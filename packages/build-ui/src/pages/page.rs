#![allow(unused)]
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

#[component]
pub fn RootPage(lang: Language) -> Element {
    let tr: RootTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col h-full text-white bg-gradient-to-b from-[#000000] to-[#1A1A1A]",
            "Root"
        }
    }
}
