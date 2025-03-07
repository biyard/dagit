use dioxus::prelude::*;
use dioxus_translate::{Language, translate};
mod i18n;

use crate::{components::ServiceLogo, routes::Route};
use i18n::HeaderTranslate;
#[component]
pub fn Header(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    rsx! {
        nav { class: "w-full flex justify-between items-center bg-transparent py-4 text-white text-base px-1 z-100",
            ServiceLogo { width: "110", height: "24", class: "fill-white" }
            div { class: "flex gap-x-10 ",
                Link { to: Route::RootPage { lang }, "{tr.solution}" }
                Link { to: Route::RootPage { lang }, "{tr.pricing}" }
                Link { to: Route::RootPage { lang }, "{tr.faq}" }
            }
            // span { "{tr.login}" }
            div { class: "flex flex-row items-center",
                div { class: "px-7.5 py-2.5 flex justify-center items-center gap-x-2.5",
                    div { class: "rounded-full bg-white w-10 h-10" }
                    span { "USER NAME" }
                }
                div {
                    Link {
                        class: "bg-transparent border border-white inline-block px-4.5 py-3 text-base text-white",
                        to: Route::HomePage {
                            lang,
                            agit_id: 0,
                        },
                        "{tr.my_agit}"
                    }
                }
            }
        }
    }
}
