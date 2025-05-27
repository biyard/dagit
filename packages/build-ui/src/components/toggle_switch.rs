use bdk::prelude::*;

#[component]
pub fn ToggleSwitch(
    #[props(default = false)] disabled: bool,
    value: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        div {
            class: "group block p-0.5 w-7 bg-neutral-80 aria-checked:bg-primary rounded-full cursor-pointer aria-disabled:cursor-not-allowed transition-colors duration-500 ease-in-out",
            aria_checked: value,
            aria_disabled: disabled,
            onclick: move |_| {
                if !disabled {
                    on_change.call(!value);
                }
            },
            div { class: "size-3 rounded-full bg-white group-aria-disabled:bg-neutral-70 group-aria-checked:translate-x-3 transition-transform duration-300" }
        }
    }
}
