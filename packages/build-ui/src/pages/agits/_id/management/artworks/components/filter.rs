use bdk::prelude::*;
use by_components::icons::arrows::ChevronUp;
use by_components::icons::{arrows, edit};

use crate::components::dropdown::DropDown;

#[component]
pub fn FilterSidebar(
    #[props(default = String::default())] class: String,
    #[props(default = EventHandler::default())] on_artist_change: EventHandler<String>,
    #[props(default = EventHandler::default())] on_price_change: EventHandler<String>,
    #[props(default = EventHandler::default())] on_attribute_change: EventHandler<String>,
) -> Element {
    let mut artist_name = use_signal(|| String::default());
    rsx! {
        div { class: "flex flex-col w-55 max-h-screen overflow-y-scroll pb-7.5 border border-neutral-80 px-4 text-white {class}",
            Accordion { label: "Artist", default_open: true,
                InputWithIcon {
                    placeholder: "Artist Name",
                    value: artist_name(),
                    oninput: move |value: String| {
                        artist_name.set(value.clone());
                        on_artist_change(value);
                    },
                
                }
            }

            Accordion { label: "Price", default_open: true,
                DropDown {
                    class: "[&>div]:!p-2 [&>div]:text-[13px]/[23px]",
                    id: "price-dropdown",
                    placeholder: "Price",
                    options: vec!["ETH".to_string(), "MATIC".to_string()],
                    onchange: move |value| on_price_change(value),
                }
            }

            Accordion { label: "Attributes", default_open: true,
                InputWithIcon {
                    placeholder: "Tag",
                    value: String::default(),
                    oninput: move |value: String| on_attribute_change(value),
                    disabled: false,
                }
            }
        }
    }
}

// #[component]
// fn DropDown(options: Vec<String>, onselect: EventHandler<String>) -> Element {
//     let mut selected = use_signal(|| options.first().cloned().unwrap_or_default());
//     rsx! {
//         div { class: "relative w-full",
//             select {
//                 class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary",
//                 value: selected(),
//                 oninput: move |e| {
//                     let value = e.value();
//                     selected.set(value.clone());
//                     onselect(value);
//                 },
//                 for option in options.iter() {
//                     option { value: "{option}", {option.clone()} }
//                 }

//             }
//         }
//     }
// }
#[component]
fn Accordion(
    #[props(default = false)] default_open: bool,
    label: String,
    children: Element,
) -> Element {
    let mut expand = use_signal(|| default_open);
    rsx! {
        div { class: "flex group flex-col", aria_expanded: !expand(),
            div {
                class: "flex flex-row mt-7.5",
                onclick: move |_| {
                    expand.toggle();
                },
                p { class: "text-[15px]/[23px] w-full flex justify-between items-center",
                    {label}
                }
                div { class: "group-aria-expanded:rotate-180 transition-transform duration-300 ease-in-out",
                    ChevronUp {
                        width: "18px",
                        height: "18px",
                        class: "[&>path]:stroke-white",
                    }
                }
            }
            div { class: "flex flex-col group-aria-expanded:hidden mt-2.5", {children} }
        }
    }
}

#[component]
fn InputWithIcon(
    placeholder: String,
    value: String,
    oninput: EventHandler<String>,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        div {
            class: "group flex border border-neutral-80 focus:border-primary p-2",
            aria_disabled: disabled,
            input {
                class: "text-[13px]/[23px] outline-none text-white placeholder-neutral-80",
                placeholder,
                value,
                disabled,
                oninput: move |e| oninput(e.value().clone()),
            }
            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
        }
    }
}
#[component]
pub fn FilterDropdown(label: String, options: Vec<String>) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col border-b-1 border-border-primary py-2 text-sm cursor-pointer hover:bg-border-bg",
            onclick: move |_| is_open.toggle(),
            div { class: "flex items-center justify-between",
                span { "{label}" }
                arrows::ChevronDown { class: "[&>path]:stroke-white", height: 18, width: 18 }
            }
            if (*is_open)() {
                div { class: "mt-2 bg-border-bg border border-border-primary rounded-md shadow-md",
                    for option in options.iter().cloned() {
                        div {
                            class: "px-4 py-2 hover:bg-primary cursor-pointer",
                            onclick: move |_| tracing::debug!("Selected: {option}"),
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}
