use bdk::prelude::*;
use by_components::icons::layouts;

#[component]
pub fn ArtButton(
    #[props(default = false)] show: bool,
    #[props(default = None)] on_click: Option<EventHandler<()>>,
) -> Element {
    if !show {
        return rsx! {
            div {}
        };
    }

    rsx! {
        button {
            class: "p-2 border border-neutral-80 outline-none text-white hover:border-primary w-full sm:w-auto",
            onclick: move |_| {
                if let Some(on_click) = &on_click {
                    on_click(());
                }
            },
            layouts::Window { class: "[&>path]:stroke-white" }
        }
    }
}
