use bdk::prelude::*;

#[component]
pub fn OptionList(
    options: Vec<String>,
    #[props(default = false)] multi_select: bool,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    let mut selected = use_signal(|| vec![]);
    rsx! {
        div { class: "w-full space-y-2",
            for option in options.iter() {
                Item {
                    multi_select,
                    value: option.clone(),
                    onchange: move |value: String| {
                        if selected().contains(&value) {
                            selected.write().retain(|v| v != &value);
                        } else {
                            selected.push(value);
                        }
                    },
                    span { class: "flex-1 text-right text-neutral-500", "{option}" }
                }
            }
        }
    }
}

#[component]
pub fn Item(
    multi_select: bool,
    value: String,
    onchange: EventHandler<String>,
    children: Element,
) -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        label { class: "flex items-center justify-between px-3 py-2 rounded cursor-pointer hover:bg-primary",
            div { class: "flex items-center gap-2 flex-1",
                input {
                    r#type: if multi_select { "checkbox" } else { "radio" },
                    name: "option",
                    // class: "w-5 h-5 bg-black border-gray-500 focus:ring-0",
                    class: "hidden",
                    onchange: move |e| {
                        let v = e.value() == "true";
                        checked.set(v);
                        onchange.call(value.clone());
                    },
                }
                div { class: "w-5 h-5 flex items-center justify-center border border-neutral-500 rounded",
                    if checked() {
                        span { class: "text-white text-lg", "/" }
                    }
                }
                {children}
            }
        }
    }
}
