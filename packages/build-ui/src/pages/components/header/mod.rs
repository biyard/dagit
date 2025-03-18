use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{Language, translate};
mod i18n;

use crate::{components::ServiceLogo, pages::{self, components::{login::LoginPage, PopupZone}}, routes::Route};
use i18n::HeaderTranslate;

#[component]
pub fn Header(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();

    rsx! {
        nav { 
            class: "w-full flex justify-between items-center bg-transparent py-4 text-white text-base px-1 z-100",

            // Logo
            ServiceLogo { width: "110", height: "24", class: "fill-white" }

            // Navigation Links
            div { class: "flex gap-x-10",
                Link { to: Route::RootPage { lang }, "{tr.solution}" }
                Link { to: Route::RootPage { lang }, "{tr.pricing}" }
                Link { to: Route::RootPage { lang }, "{tr.faq}" }
            }

            // User Actions
            div { class: "flex flex-row items-center",
                div { class: "px-7.5 py-2.5 flex justify-center items-center gap-x-2.5" }

                // Login Button (Triggers Popup)
                button {
                    class: "bg-transparent border border-white px-4.5 py-3 text-base text-white",
                    onclick: move |_| {
                        popup.open(LoginPage(pages::components::login::LoginPageProps { lang: lang.clone() }));
                    },
                    "{tr.my_agit}"
                }
            }
        }
    }
}
