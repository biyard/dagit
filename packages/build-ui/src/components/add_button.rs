use bdk::prelude::*;
use by_components::icons::{folder, validations};
#[component]
pub fn AddButton(
    #[props(default = false)] show: bool,
    #[props(default = None)] on_click: Option<EventHandler<()>>,
    #[props(default = "".to_string())] text: String,
    #[props(default = "".to_string())] remove_text: String,
) -> Element {
    if !show {
        return rsx! {
            div {}
        };
    }

    rsx! {
        button {
            class: "bg-black border border-white text-white p-2.5 flex items-center justify-center w-full sm:w-auto hover:border-primary focus:border-primary",
            onclick: move |_| {
                if let Some(on_click) = &on_click {
                    on_click(());
                }
            },
            if remove_text.is_empty() {
                folder::UploadFolder { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
            } else {
                validations::Add { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
            }
            "{text}"
        }
    }
}
