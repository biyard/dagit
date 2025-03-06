#![allow(non_snake_case)]
use by_components::loaders::cube_loader::CubeLoader;
use dioxus::prelude::*;

use dioxus_translate::Language;

use super::components::PopupZone;
use crate::routes::Route;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center bg-primary",
            PopupZone {}
            SuspenseBoundary {
                fallback: |_| {
                    rsx! {
                        div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                            CubeLoader {}
                        }
                    }
                },
                div { class: "max-w-[1440px] w-full flex flex-row",
                    div { Outlet::<Route> {} }
                }
            }
        }
    }
}
