use dioxus::prelude::*;
#[component]
pub fn FilterDropdown(label: String) -> Element {
    rsx! {
        div { class: "flex items-center border-b-1 border-[#333] justify-between py-2 text-sm cursor-pointer hover:bg-[#222]",
    
            span { "{label}" }
            svg {
                view_box: "0 0 24 24",
                width: "16",
                height: "16",
                stroke: "currentColor",
                stroke_width: "2",
                fill: "none",
                class: "transform transition-transform",
                path {
                    d: "M19 9l-7 7-7-7"
                }
            }
        }
    }
}
