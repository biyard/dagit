#![allow(non_snake_case)]
use bdk::prelude::*;

pub mod config;
pub mod layout;

pub mod components;
pub mod pages;
pub mod routes;
pub mod utils;
use routes::Route;

const FAVICON: Asset = asset!("/public/favicon.svg");
const MAIN_CSS: Asset = asset!("/public/main.css");
const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    dioxus_aws::launch(App);
}

#[component]
fn App() -> Element {
    let css = include_str!("../public/theme.css");

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css",
        }
        document::Style { r#type: "text/tailwindcss", {css} }
        document::Script { src: "https://unpkg.com/@tailwindcss/browser@4.0.12/dist/index.global.js" }
        Router::<Route> {}
    }
}
