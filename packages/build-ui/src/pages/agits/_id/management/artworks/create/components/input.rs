use bdk::prelude::{by_components::icons::security::Lock2, *};

#[component]
pub fn TitleInput(
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    on_go_back: EventHandler<()>,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        div { class: "flex gap-2 w-full items-center",
            div { class: "size-7",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "11",
                    height: "23",
                    view_box: "0 0 11 23",
                    fill: "none",
                    onclick: move |_| {
                        on_go_back.call(());
                    },
                    path {
                        d: "M9.95313 1.75781L1.95312 11.7578L9.95313 21.7578",
                        stroke: "white",
                        stroke_width: "2",
                        stroke_linecap: "round",
                    }
                
                }
            }
            input {
                class: "text-[32px]/[38px] pb-1 font-semibold flex-1 border-b border-white outline-none text-white hover:border-primary focus:border-primary placeholder-neutral-80",
                placeholder,
                value,
                disabled,
                oninput: move |e| on_change(e.value().clone()),
            }
            Lock2 { class: "[&>path]:stroke-white" }
        
        }
    }
}
