#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Chat {},
}

// const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Chat() -> Element {
    let mut user_speech = use_signal(|| "default".to_string());

    rsx! {
        div {
            input {
                // we tell the component what to render
                value: "{user_speech}",
            }
        }
        div {
            h1 { "Speech Recognition" }
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 border border-blue-700 rounded",
                onclick: move |_| user_speech.set(speak()),
                "Speak!"
            }
        }
    }
}

fn speak() -> String {
    return "hello roydy!!".to_string()
}

// #[wasm_bindgen(start)]
// pub fn speak() -> Result<(), JsValue> {
//     // Get the window and document objects
//     let window = window().expect("should have a window in this context");
//     let document = window.document().expect("window should have a document");

//     // Create a paragraph element to display recognized text
//     let output = document.create_element("p")?;
//     let output = output.dyn_into::<web_sys::HtmlElement>()?;
//     document.body().expect("document should have a body").append_child(&output)?;

//     // Initialize SpeechRecognition
//     let recognition = SpeechRecognition::new()?;
//     let recognition = recognition
//         .dyn_into::<web_sys::SpeechRecognition>()?;

//     // Configure SpeechRecognition
//     recognition.lang = "en-US".to_string(); // Set language
//     recognition.interim_results = true; // Enable interim results

//     // Event listener for speech recognition
//     let onresult_callback = Closure::wrap(Box::new(move |event: SpeechRecognitionEvent| {
//         let transcript = event.results()
//             .item(0)
//             .and_then(|result| result.item(0))
//             .map(|result| result.transcript())
//             .unwrap_or_else(|| "".to_string());

//         output.set_text_content(Some(&transcript));
//     }) as Box<dyn FnMut(_)>);

//     recognition.set_onresult(Some(onresult_callback.as_ref().unchecked_ref()));

//     // Error handling
//     let onerror_callback = Closure::wrap(Box::new(move |error: SpeechRecognitionError| {
//         console_error!("Speech recognition error: {:?}", error);
//     }) as Box<dyn FnMut(_)>);
//     recognition.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

//     // Start speech recognition
//     recognition.start()?;

//     Ok(())
// }
