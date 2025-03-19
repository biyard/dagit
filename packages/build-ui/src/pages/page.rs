#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

use crate::{components::ServiceLogo, routes::Route};

#[component]
pub fn RootPage(lang: Language) -> Element {
    // TODO - Implement AuthContext.
    let logged_in = use_signal(|| false); 

    rsx! {
        div {
            class: "text-white flex flex-col items-center justify-center min-h-screen p-6",
            
            if *logged_in.read() {
                div { class: "text-2xl font-bold", "Home" }
            } else {
                div {
                    class: "flex flex-col items-center gap-6 max-w-2xl text-center",

                    ServiceLogo { width: "110", height: "24", class: "fill-white" }

                    div {
                        class: "text-3xl font-bold",
                        "Blockchain-based artwork certificates, seamless data management, digital gallery solutions."
                    }

                    div {
                        class: "text-lg text-gray-300",
                        "Let's unlock new possibilities with your own Agit today!"
                    }

                    button {
                        class: "px-6 py-3 mt-4 bg-[#30D4A0] text-black font-bold rounded-lg cursor-pointer shadow-md hover:bg-[#30D4A0] transition",
                        "Build your Agit"
                    }

                }
            }
        }
    }
}
