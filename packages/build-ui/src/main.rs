#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod components;
pub mod config;
pub mod pages;
pub mod routes;
use dioxus_popup::PopupService;
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
    PopupService::init();
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
#[component]
fn load_tailwindcss() -> Element {
    use dioxus::document::{StyleProps, document};

    let theme = include_str!("../tailwind-theme.css");
    let v = StyleProps::builder()
        .children(rsx! {
            {theme}
        })
        .r#type("text/tailwindcss")
        .build();
    let doc = document();
    doc.create_style(v);
    // Note:
    // `style { r#type: "text/tailwindcss", {theme}}` is not working.
    // The reason is that Dioxus creates `<!-- -->` comment nodes,
    // which are not allowed inside the `<style>` tag.
    rsx! {
        script { src: "https://unpkg.com/@tailwindcss/browser@4" }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
#[component]
fn load_tailwindcss() -> Element {
    VNode::empty()
}
