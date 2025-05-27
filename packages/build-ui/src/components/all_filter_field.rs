use bdk::prelude::*;
use by_components::icons::arrows;

#[component]
pub fn AllFilterField(#[props(default = false)] show: bool) -> Element {
    if !show {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "relative",
            div { class: "absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none",
                arrows::ChevronDown { class: "[&>path]:stroke-white", height: 20, width: 20 }
            }
            input {
                class: "text-[15px]/[23px] border border-neutral-80 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-800 disabled:!border-neutral-80 pl-3 p-2.5",
                placeholder: "All",
                r#type: "text",
            }
        }
    }
}
