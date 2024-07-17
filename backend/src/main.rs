use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use dotenv::dotenv;
use std::env;
use env_logger;
use log::info;

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HuggingFaceRequest {
    inputs: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HuggingFaceResponse {
    generated_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    response: String,
}

async fn chat_handler(req: web::Json<ChatRequest>, client: web::Data<Client>, api_key: web::Data<String>) -> impl Responder {
    info!("Received request: {:?}", req);

    let hugging_face_request = HuggingFaceRequest {
        inputs: req.message.clone(),
    };

    let response = client.post("https://api-inference.huggingface.co/models/meta-llama/Meta-Llama-3-8B")
        .header("Authorization", format!("Bearer {}", api_key.get_ref()))
        .json(&hugging_face_request)
        .send()
        .await;

    match response {
        Ok(response) => {
            info!("Received response from Hugging Face API");
            let text = response.text().await;
            match text {
                Ok(text) => {
                    info!("Raw text from Hugging Face API: {}", text);
                    let hugging_face_response: Result<HuggingFaceResponse, _> = serde_json::from_str(&text);
                    match hugging_face_response {
                        Ok(parsed_response) => {
                            info!("Parsed response from Hugging Face API: {:?}", parsed_response);
                            let chat_response = ChatResponse {
                                response: parsed_response.generated_text.unwrap_or_else(|| "No generated text found".to_string()),
                            };
                            HttpResponse::Ok().json(chat_response)
                        }
                        Err(_) => {
                            HttpResponse::InternalServerError().body(format!("Error: Could not parse Hugging Face API response: {}", text))
                        }
                    }
                }
                Err(err) => {
                    HttpResponse::InternalServerError().body(format!("Error: Failed to read Hugging Face API response: {:?}", err))
                }
            }
        }
        Err(err) => {
            info!("Failed to contact Hugging Face API: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Error: Failed to contact Hugging Face API: {:?}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let api_key = env::var("HUGGING_FACE_API_KEY")
        .expect("HUGGING_FACE_API_KEY must be set in .env file");

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(api_key.clone()))
            .route("/chat", web::post().to(chat_handler))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
