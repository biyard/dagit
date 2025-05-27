use bdk::prelude::*;
use by_components::icons::edit;
#[component]
pub fn SearchInput(
    #[props(default=String::default())] class: String,
    placeholder: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "flex gap-2 p-2.5 border justify-center items-center border-neutral-80 focus-within:border-primary {class}",
            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
            input {
                class: "flex-1 placeholder:text-neutral-80 outline-none text-white font-medium text-base disabled:!border-neutral-80",
                placeholder,
                r#type: "text",
                oninput: move |e| on_change(e.value()),
            }
        }
    }
}
