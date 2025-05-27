use bdk::prelude::*;

#[component]
pub fn Input(
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] invalid_message: String,
    #[props(default = false)] readonly: bool,
) -> Element {
    rsx! {
        div { class: "flex w-full",
            input {
                "aria-invalid": invalid,
                class: "flex-1 text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-80 disabled:!border-neutral-80",
                placeholder,
                value,
                disabled,
                readonly,
                oninput: move |e| on_change(e.value().clone()),
            }
        }
        if invalid {
            span { class: "text-[15px]/[23px] text-pink", {invalid_message} }
        }
    }
}

#[component]
pub fn InputWithLabel(
    label: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] invalid_message: String,
    #[props(default = false)] readonly: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2", ..attributes,
            label { class: "text-sm/relaxed font-semibold text-neutral-70", {label} }
            Input {
                placeholder,
                value,
                on_change,
                disabled,
                invalid,
                invalid_message,
                readonly,
            }
        }
    }
}

#[component]
pub fn Input2(
    label: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] invalid_message: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "flex-1".to_string())] width_class: String,
) -> Element {
    rsx! {
        div { class: "flex items-center", ..attributes,
            label { class: "text-sm/relaxed font-semibold text-white w-50 flex items-center", // Align label vertically
                span { class: "text-red-500 mr-1", "*" }
                {label}
            }
            input {
                "aria-invalid": invalid,
                class: format!(
                    "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-80 disabled:!border-neutral-80 {}",
                    width_class,
                ),
                placeholder,
                value,
                disabled,
                oninput: move |e| on_change(e.value().clone()),
            }
            if invalid {
                span { class: "text-[15px]/[23px] text-pink", {invalid_message} }
            }
        }
    }
}

#[component]
pub fn TextArea(
    label: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2", ..attributes,
            label { class: "text-xl/relaxed font-semibold text-white", {label} }
            textarea {
                "aria-invalid": invalid,
                class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80",
                placeholder,
                value,
                disabled,
                oninput: move |e| on_change(e.value().clone()),
            }
        }
    }
}
