#![allow(non_snake_case)]
use bdk::prelude::*;

pub mod components;
pub mod config;
pub mod pages;
pub mod routes;
pub mod services;

use dioxus_oauth::prelude::FirebaseProvider;
use dioxus_popup::PopupService;
use routes::Route;
use services::user_service::UserService;

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
    UserService::init()?;
    let css = include_str!("../public/theme.css");
    let conf = config::get();
    rsx! {
        btracing::ToastTracing {}
        FirebaseProvider {
            api_key: conf.firebase.api_key.clone(),
            auth_domain: conf.firebase.auth_domain.clone(),
            project_id: conf.firebase.project_id.clone(),
            storage_bucket: conf.firebase.storage_bucket.clone(),
            messaging_sender_id: conf.firebase.messaging_sender_id.clone(),
            app_id: conf.firebase.app_id.clone(),
            measurement_id: conf.firebase.measurement_id.clone(),
        }
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
