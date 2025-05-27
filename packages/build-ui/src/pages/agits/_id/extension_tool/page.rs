use bdk::prelude::*;

use crate::pages::agits::components::ComingSoon;

#[component]
pub fn ExtensionToolPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        ComingSoon {}
    }
}
