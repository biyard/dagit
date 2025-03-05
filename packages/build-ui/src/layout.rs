#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_translate::Language;

use crate::routes::Route;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center bg-[#F3F3F3]", Outlet::<Route> {} }
    }
}
