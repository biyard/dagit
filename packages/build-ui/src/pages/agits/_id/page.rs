use bdk::prelude::*;

#[component]
pub fn HomePage(lang: Language, agit_id: i64) -> Element {
    rsx! {
        div { class: "flex flex-col h-full text-white", "Home {agit_id}" }
    }
}
