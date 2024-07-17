use yew::prelude::*;
use yew::platform::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
                new_messages.push(("user", input_message.clone()));
                messages.set(new_messages.clone());
                input_value.set(String::new());

                let messages_clone = messages.clone();
                spawn_local(async move {
                    let request_body = ChatRequest {
                        message: input_message.clone(),
                    };
                    match Request::post("http://127.0.0.1:8081/chat")
                        .json(&request_body)
                        .unwrap()
                        .send()
                        .await {
                        Ok(response) => {
                            if let Ok(chat_response) = response.json::<ChatResponse>().await {
                                let mut updated_messages = new_messages.clone();
                                updated_messages.push(("bot", chat_response.response));
                                messages_clone.set(updated_messages);
                            }
                        }
                        Err(err) => {
                            let mut updated_messages = new_messages.clone();
                            updated_messages.push(("bot", format!("Error: {}", err)));
                            messages_clone.set(updated_messages);
                        }
                    }
                });
            }
        })
    };

    let onclick = {
        let input_value = input_value.clone();
        let messages = messages.clone();
        Callback::from(move |_| {
            let input_message = (*input_value).clone();
            let mut new_messages = (*messages).clone();
            new_messages.push(("user", input_message.clone()));
            messages.set(new_messages.clone());
            input_value.set(String::new());

            let messages_clone = messages.clone();
            spawn_local(async move {
                let request_body = ChatRequest {
                    message: input_message.clone(),
                };
                match Request::post("http://127.0.0.1:8081/chat")
                    .json(&request_body)
                    .unwrap()
                    .send()
                    .await {
                    Ok(response) => {
                        if let Ok(chat_response) = response.json::<ChatResponse>().await {
                            let mut updated_messages = new_messages.clone();
                            updated_messages.push(("bot", chat_response.response));
                            messages_clone.set(updated_messages);
                        }
                    }
                    Err(err) => {
                        let mut updated_messages = new_messages.clone();
                        updated_messages.push(("bot", format!("Error: {}", err)));
                        messages_clone.set(updated_messages);
                    }
                }
            });
        })
    };

    html! {
        <div class="chatbot-container">
            <div class="header">
                { "Chatbot" }
            </div>
            <div class="messages">
                { for (*messages).iter().map(|(sender, message)| html! {
                    <div class={classes!("message", *sender)}>{ message }</div>
                })}
            </div>
            <div class="message-input-container">
                <input
                    class="message-input"
                    type="text"
                    value={(*input_value).clone()}
                    {oninput}
                    {onkeypress}
                    placeholder="Type your message"
                />
                <button class="send-button" {onclick}>{ "Send" }</button>
            </div>
        </div>
    }
}
