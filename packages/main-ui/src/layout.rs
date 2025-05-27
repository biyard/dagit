#![allow(non_snake_case)]
use crate::routes::Route;
use bdk::prelude::*;

use crate::components::prelude::*;

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
