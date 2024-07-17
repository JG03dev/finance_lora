use yew::prelude::*;
use wasm_bindgen::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <components::chatbot::Chatbot />
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
