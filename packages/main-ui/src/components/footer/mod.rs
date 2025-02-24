use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::routes::Route;

#[component]
pub fn Footer(lang: Language) -> Element {
    rsx! {
        footer { class: "flex gap-30 py-10 justify-center items-center text-sm font-semibold",
            div { "© 2025 Biyard" }
            Link {
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                "Terms and Conditions"
            }
            Link {
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                "Privacy"
            }
            Link {
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                "Policy"
            }
            Link {
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                "Security"
            }
        }
    }
}
