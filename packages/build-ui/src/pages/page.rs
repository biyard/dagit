#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

use crate::routes::Route;

#[component]
pub fn RootPage(lang: Language) -> Element {
    rsx! {
        div { class: "text-white", "Home" }
    }
}
