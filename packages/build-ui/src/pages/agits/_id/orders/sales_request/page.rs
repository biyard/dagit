use bdk::prelude::*;

use crate::pages::agits::components::ComingSoon;

#[component]
pub fn SalesRequestPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    rsx! {
        ComingSoon {}
    }
}
