#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_translate::Language;

use crate::{components::prelude::{Footer, Header}, routes::Route};


#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center bg-[#F3F3F3]",
            Header { lang: lang.clone() }
            div { class: "w-full flex flex-col min-h-lvh mt-23",
                Outlet::<Route> {}
                Footer { lang: lang.clone() }
            }
        }
    }
}
