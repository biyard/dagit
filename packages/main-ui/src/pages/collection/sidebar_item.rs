use dioxus::prelude::*;
#[component]

pub fn SidebarItem(label: String, active: bool, indent: Option<bool>) -> Element {
    let indent_class = if indent.unwrap_or(false) { "pl-8" } else { "pl-4" };
    let active_class = if active { "bg-[#333]" } else { "" };
    
    rsx! {
        div { class: "py-2 {indent_class} {active_class} hover:bg-[#333] cursor-pointer",
            "{label}"
        }
    }
}