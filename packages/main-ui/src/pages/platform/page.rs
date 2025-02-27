#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;

#[component]
pub fn PlatformPage(lang: Language) -> Element {
    rsx! {
        div {
            class: "min-h-screen text-white flex flex-col bg-gradient-to-b from-[#000000] to-[#1A1A1A]",

            nav {
                class: "flex items-center justify-between px-8 py-4",

                h1 {
                    class: "text-2xl font-bold",
                    "d.AGIT"
                }

                div {
                    class: "hidden md:flex space-x-4",
                    a {
                        class: "hover:text-[#30D4A0]",
                        href: "#solution",
                        "Solution"
                    }
                    a {
                        class: "hover:text-[#30D4A0]",
                        href: "#pricing",
                        "Pricing"
                    }
                    a {
                        class: "hover:text-[#30D4A0]",
                        href: "#faq",
                        "FAQ"
                    }
                }

                a {
                    class: "hover:text-[#30D4A0]",
                    href: "#login",
                    "Login"
                }
            }

            div {
                class: "flex flex-col items-center justify-center flex-grow text-center px-4",

                h2 {
                    class: "text-4xl md:text-6xl font-extrabold mb-4",
                    "d.AGIT"
                }

                p {
                    class: "text-4xl md:text-4xl max-w-6xl mb-6",
                    "Blockchain-based artwork certificates, seamless data management, digital gallery solutions."
                }

                p {
                    class: "text-md md:text-lg max-w-xl mb-8",
                    "Let's unlock new possibilities with your own Agit today!"
                }

                button {
                    class: "bg-[#30D4A0] text-black px-6 py-3 rounded-sm font-semibold hover:bg-gray-400 transition",
                    "Build your Agit"
                }
            }
        }
    }
}
