use dioxus::prelude::*;

pub mod config;
pub mod layout;

pub mod components;
pub mod pages;
pub mod routes;
pub mod utils;
use routes::Route;
const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    dioxus_aws::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css",
        }
        load_tailwindcss {}
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        script { src: "https://unpkg.com/@tailwindcss/browser@4" }
    }
}

// #[cfg(feature = "lambda")]
// #[allow(dead_code)]
// fn load_tailwindcss() -> Element {
//     rsx! {}
// }
