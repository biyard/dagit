#![allow(unused)]
use super::i18n::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

#[component]
pub fn MainPage(lang: Language) -> Element {
    let tr: PlatformTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col h-full text-white bg-gradient-to-b from-[#000000] to-[#1A1A1A]",

            nav { class: "flex items-center justify-between px-8 py-4",

                h1 { class: "text-2xl font-bold", "{tr.d_agit}" }

                div { class: "hidden md:flex space-x-4",
                    a { class: "hover:text-[#30D4A0]", href: "#solution", "{tr.solution}" }
                    a { class: "hover:text-[#30D4A0]", href: "#pricing", "{tr.pricing}" }
                    a { class: "hover:text-[#30D4A0]", href: "#faq", "{tr.faq}" }
                }

                a { class: "hover:text-[#30D4A0]", href: "#login", "{tr.login}" }
            }

            div { class: "flex flex-col items-center justify-center flex-grow text-center px-4",

                h2 { class: "text-4xl md:text-6xl font-extrabold mb-4", "{tr.d_agit}" }

                p { class: "text-4xl md:text-4xl max-w-6xl mb-6", "{tr.description}" }

                p { class: "text-md md:text-lg max-w-xl mb-8", "{tr.unlock_possibilities}" }

                button { class: "bg-[#30D4A0] text-black px-6 py-3 rounded-sm font-semibold hover:bg-gray-400 transition",
                    "{tr.build_agit}"
                }
            }
        }
    }
}
