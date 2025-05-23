use bdk::prelude::{by_components::icons::arrows::ChevronDown, *};

#[component]
pub fn DropDown(
    #[props(default = String::default())] class: String,
    default_value: String,
    on_change: EventHandler<String>,
    options: Vec<String>,
) -> Element {
    let value = use_signal(|| default_value);

    rsx! {
        div { class: "flex border border-neutral-80 items-center text-white px-4 py-3 {class}",
            div { class: "flex-1", {value()} }
            ChevronDown { class: "[&>path]:stroke-white" }
        }
    }
}
