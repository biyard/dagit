use bdk::prelude::*;
use by_components::icons::{arrows, edit};

use crate::components::checkbox::CheckBoxWithLabel;

#[component]
pub fn FilterSidebar(
    #[props(default = EventHandler::default())] on_artist_change: EventHandler<String>,
    #[props(default = EventHandler::default())] on_price_change: EventHandler<String>,
    #[props(default = EventHandler::default())] on_attribute_change: EventHandler<String>,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] artist_input_placeholder: String,
    #[props(default = "".to_string())] attribute_input_placeholder: String,
    #[props(default = false)] disabled: bool,
    value: String,
    attribute_value: String,
) -> Element {
    rsx! {
        div { class: "w-64 bg-black border-neutral-80 border p-4 flex flex-col gap-6",
            // Artist Section
            div { id: "artist-section", class: "flex flex-col gap-2",
                label { class: "text-sm/relaxed font-semibold", "Artist" }
                div { class: "relative flex",
                    input {
                        "aria-invalid": invalid,
                        class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80 pr-10", // Added padding-right to make space for the icon
                        placeholder: artist_input_placeholder,
                        value,
                        disabled,
                        oninput: move |e| on_artist_change(e.value().clone()),
                    }
                    div { class: "absolute inset-y-0 right-3 flex items-center pointer-events-none", // Positioned the icon to the right inside the input
                        edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                    }
                }
            }

            // Price Section
            div { id: "price-section", class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Price" }
                select {
                    class: "text-[15px]/23 border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80",
                    oninput: move |e| on_price_change(e.value().clone()), // Added event handler for dynamic options
                    option { value: "", disabled: true, selected: true, "Select a currency" } // Placeholder option
                    for currency in ["ETH", "MATIC"].iter() {
                        option { value: "{currency}", "{currency}" }
                    }
                }
                div { class: "flex gap-2",
                    button { class: "flex-1 bg-border-bg border border-border-primary text-white text-sm p-2 rounded-3xl",
                        "Lowest"
                    }
                    button { class: "flex-1 bg-border-bg border border-border-primary text-white text-sm p-2 rounded-3xl",
                        "Highest"
                    }
                }
            }

            // Attributes Section
            div { id: "attributes-section", class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Attributes" }
                div { class: "relative flex",
                    input {
                        "aria-invalid": invalid,
                        class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80 pr-10", // Added padding-right to make space for the icon
                        placeholder: attribute_input_placeholder,
                        value: attribute_value,
                        disabled,
                        oninput: move |e| on_attribute_change(e.value().clone()),
                    }
                    div { class: "absolute inset-y-0 right-3 flex items-center pointer-events-none", // Positioned the icon to the right inside the input
                        edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                    }
                }
                //Fixme: The option list here are hardcoded
                // Attribute dropdowns
                FilterDropdown {
                    label: "Medium",
                    options: vec!["Oil".to_string(), "Acrylic".to_string(), "Watercolor".to_string()],
                }
                FilterDropdown {
                    label: "Theme",
                    options: vec!["Abstract".to_string(), "Nature".to_string(), "Portrait".to_string()],
                }
                FilterDropdown {
                    label: "Art style",
                    options: vec!["Modern".to_string(), "Classic".to_string(), "Impressionist".to_string()],
                }
                FilterDropdown {
                    label: "Material",
                    options: vec!["Canvas".to_string(), "Paper".to_string(), "Wood".to_string()],
                }
                FilterDropdown {
                    label: "Color",
                    options: vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()],
                }
                FilterDropdown {
                    label: "Size",
                    options: vec!["Small".to_string(), "Medium".to_string(), "Large".to_string()],
                }
                FilterDropdown {
                    label: "Year",
                    options: vec!["2020".to_string(), "2021".to_string(), "2022".to_string()],
                }
            }
            // Fixme:
            // Tags Section
            div { class: "flex flex-col gap-2",
                CheckBoxWithLabel {
                    label: "Tag".to_string(),
                    on_change: move |_| {
                        tracing::debug!("Checkbox clicked");
                    },
                }
                CheckBoxWithLabel {
                    label: "Tag".to_string(),
                    on_change: move |_| {
                        tracing::debug!("Checkbox clicked");
                    },
                }
                CheckBoxWithLabel {
                    label: "Tag".to_string(),
                    on_change: move |_| {
                        tracing::debug!("Checkbox clicked");
                    },
                }
            }
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
