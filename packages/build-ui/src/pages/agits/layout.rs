#![allow(non_snake_case)]
use crate::{pages::layout::SuspenseWrapper, routes::Route};
use dioxus::prelude::*;
use dioxus_translate::Language;

use super::Navigation;

#[component]
pub fn NavigationLayout(lang: Language, agit_id: i64) -> Element {
    rsx! {
        SuspenseWrapper {
            div { class: "flex flex-row bg-background min-h-svh",
                Navigation { lang, agit_id }
                div { class: "py-10", Outlet::<Route> {} }
            }
        }
    }
}
