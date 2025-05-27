use bdk::prelude::*;
use by_components::icons::validations;

#[component]
pub fn RemoveButton(
    #[props(default = false)] show: bool,
    #[props(default = None)] on_click: Option<EventHandler<()>>,
    #[props(default = "".to_string())] text: String,
) -> Element {
    if !show {
        return rsx! {
            div {}
        };
    }
    rsx! {
        button {
            class: "bg-border-background border border-border-primary text-white p-2.5 flex items-center justify-center w-full sm:w-auto hover:border-primary focus:border-primary",
            onclick: move |_| {
                if let Some(on_click) = &on_click {
                    on_click(());
                }
            },
            validations::Remove { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
            "{text}"
        }
    }
}
