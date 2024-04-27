#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
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
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button {
                class: "mb-4 mr-2 text-white bg-blue-500 border-0 rounded py-1 px-4 focus:outline-none hover:bg-gray-300",
                onclick: move |_| count += 1, "Up high!",
            }
            button {
                class: "mb-4 text-white bg-blue-500 border-0 rounded py-1 px-4 focus:outline-none hover:bg-gray-300",
                onclick: move |_| count -= 1, "Down low!" ,
            }
        }
    }
}
