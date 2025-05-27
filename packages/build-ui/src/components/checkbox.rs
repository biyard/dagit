use bdk::prelude::{by_components::icons::validations::Check as CheckIcon, *};

#[component]
pub fn Checkbox(
    #[props(default = "check_box".to_string())] id: String,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "cursor-pointer flex items-center", // Add cursor-pointer for better UX

            input {
                id,
                r#type: "checkbox",
                class: "sr-only",
                checked,
                onchange: move |e| {
                    let v = e.value() == "true";
                    onchange(v);
                },
            }
            CheckIcon {
                class: format!(
                    "border {}",
                    if checked { "border-white bg-white" } else { "border-neutral-80" },
                ),
                height: 18,
                width: 18,
            }
        }
    }
}

#[component]
pub fn CheckBoxWithLabel(label: String, on_change: EventHandler<bool>) -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div {
            class: "flex flex-row justify-start items-center gap-2",
            onclick: move |_| {
                tracing::debug!("Checkbox clicked");
                checked.toggle();
                on_change.call(checked());
            },
            Checkbox { checked: checked(), onchange: move |_| {} }
            {label}
        }
    }
}
