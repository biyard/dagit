use bdk::prelude::*;

#[component]
pub fn DropdownInput(
    label: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] invalid: bool,
    #[props(default = "".to_string())] invalid_message: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    options: Vec<String>,
) -> Element {
    rsx! {
        div { class: "flex items-center ", ..attributes,
            label { class: "text-sm/relaxed font-semibold text-white w-50 flex items-center", // Align label vertically
                span { class: "text-red-500 mr-1", "*" }
                {label}
            }
            select {
                "aria-invalid": invalid,
                class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80 w-64", // Added width class
                disabled: "{disabled}",
                oninput: move |e| on_change(e.value().clone()),
                option { value: "", disabled: true, selected: value.is_empty(), "{placeholder}" } // Placeholder option
                for option in options.iter() {
                    option { value: "{option}", selected: value == *option, "{option}" }
                }
            }
            span {
                class: "text-[15px]/[23px] text-pink aria-invalid:hidden",
                "aria-invalid": invalid,
                {invalid_message}
            }
        }
    }
}
