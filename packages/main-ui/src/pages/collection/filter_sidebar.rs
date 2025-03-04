use dioxus::prelude::*;

use crate::pages::collection::FilterDropdown;

#[component]
pub fn FilterSidebar() -> Element {
    rsx! {
        div { class: "w-64 bg-[#000000] border-r border-[#333] p-4 flex flex-col gap-6",
            // Artist Section
            div { class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Artist" }
                div { class: "relative",
                    input {
                        class: "w-full bg-[#222] border border-[#333] text-white text-sm rounded-none p-2 pl-8",
                        placeholder: "Search",
                        r#type: "text"
                    }
                    div { class: "absolute inset-y-0 right-2 flex items-center pointer-events-none",
                        svg {
                            view_box: "0 0 24 24",
                            width: "16",
                            height: "16",
                            stroke: "currentColor",
                            stroke_width: "2",
                            fill: "none",
                            path {
                                d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            }
                        }
                    }
                }
            }

            // Price Section
            div { class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Price" }
                select { class: "bg-[#222] border border-[#333] text-white text-sm p-2",
                // placeholder: "Currency",
                    option { "Currency" }
                }
                div { class: "flex gap-2",
                    button { class: "flex-1 bg-[#222] border border-[#333] text-white text-sm p-2 rounded-3xl", "Lowest" }
                    button { class: "flex-1 bg-[#222] border border-[#333] text-white text-sm p-2 rounded-3xl", "Highest" }
                }
            }

            // Attributes Section
            div { class: "flex flex-col gap-2",
                div { class: "text-sm font-semibold", "Attributes" }
                div { class: "relative",
                    input {
                        class: "w-full bg-[#222] border border-[#333] text-white text-sm rounded-none p-2 pl-8",
                        placeholder: "Search",
                        r#type: "text"
                    }
                    div { class: "absolute inset-y-0 right-2 flex items-center pointer-events-none",
                        svg {
                            view_box: "0 0 24 24",
                            width: "16",
                            height: "16",
                            stroke: "currentColor",
                            stroke_width: "2",
                            fill: "none",
                            path {
                                d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            }
                        }
                    }
                }

                // Attribute dropdowns
                FilterDropdown { label: "Medium" }
                FilterDropdown { label: "Theme" }
                FilterDropdown { label: "Art style" }
                FilterDropdown { label: "Material" }
                FilterDropdown { label: "Color" }
                FilterDropdown { label: "Size" }
                FilterDropdown { label: "Year" }
            }

            // Tags Section
            div { class: "flex flex-col gap-2",
                div { class: "flex items-center justify-between text-sm text-gray-400",
                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            class: "form-checkbox bg-transparent border-[#333] text-primary rounded-none accent-[#333]"
                        }
                        span { "Tag" }
                    }
                    span { "50" }
                }
                // Repeat for more tags...
                div { class: "flex items-center justify-between text-sm text-gray-400",
                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            class: "form-checkbox bg-transparent border-[#333] text-primary rounded-none accent-[#333]"
                        }
                        span { "Tag" }
                    }
                    span { "50" }
                }
            }
        }
    }
}
