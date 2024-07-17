use yew::prelude::*;
use yew::platform::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    response: String,
}

#[function_component(Chatbot)]
pub fn chatbot() -> Html {
    let messages = use_state(Vec::new);
    let input_value = use_state(String::new);

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            input_value.set(input);
        })
    };

    let onkeypress = {
        let messages = messages.clone();
        let input_value = input_value.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input_message = (*input_value).clone();
                let mut new_messages = (*messages).clone();
                new_messages.push(format!("You: {}", input_message));
                messages.set(new_messages.clone());
                input_value.set(String::new());

                let messages_clone = messages.clone();
                spawn_local(async move {
                    let request_body = ChatRequest {
                        message: input_message.clone(),
                    };
                    match Request::post("http://localhost:8081/chat")
                        .json(&request_body)
                        .unwrap()
                        .send()
                        .await {
                        Ok(response) => {
                            if let Ok(chat_response) = response.json::<ChatResponse>().await {
                                let mut updated_messages = new_messages.clone();
                                updated_messages.push(format!("Bot: {}", chat_response.response));
                                messages_clone.set(updated_messages);
                            }
                        }
                        Err(err) => {
                            let mut updated_messages = new_messages.clone();
                            updated_messages.push(format!("Error: {}", err));
                            messages_clone.set(updated_messages);
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="chatbot-container">
            <div class="messages">
                { for (*messages).iter().map(|message| html! {
                    <div class="message">{ message }</div>
                })}
            </div>
            <input
                class="message-input"
                type="text"
                value={(*input_value).clone()}
                {oninput}
                {onkeypress}
                placeholder="Type your message and press Enter"
            />
        </div>
    }
}
