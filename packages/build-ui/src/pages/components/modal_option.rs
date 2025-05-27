use bdk::prelude::*;

#[component]
pub fn ModalOption(
    onclick: EventHandler<()>,
    #[props(default = VNode::empty())] icon: Element,
    #[props(default = "".to_string())] label: String,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        button {
            class: "min-w-100 flex flex-row gap-5 w-full px-5 py-3.5 items-center text-base font-semibold border border-neutral hover:border-primary hover:bg-primary/25 disabled:border-neutral/25 disabled:bg-neutral/25",
            disabled,
            onclick: move |_| {
                onclick(());
            },
            {icon}
            {label}
        }
    }
}
