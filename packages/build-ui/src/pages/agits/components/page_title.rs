use bdk::prelude::*;

#[component]
pub fn PageTitle(title: String, sub_title: String) -> Element {
    rsx! {
        div { class: "flex flex-col text-white mb-14",
            h1 { class: "text-[32px]/[38px] font-semibold", {title} }
            h2 { class: "text-sm font-semibold", {sub_title} }
        }
    }
}
