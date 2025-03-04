
use dioxus::prelude::*;

#[component]
pub fn SidebarSection(label: String) -> Element {

    rsx! {
        div { class: "py-2 pl-4 text-gray-400 mt-2",
            "{label}"
        }
    }
}
