// use super::controller::Controller;
// use super::i18n::CollectionTranslate;
// use crate::pages::agits::_id::management::collections::components::CollectionTable;
// #[allow(unused_imports)]
// use crate::{
//     components::{filter_sidebar::FilterSidebar, search_filter_bar::SearchFilterBar},
//     routes::Route,
// };
use bdk::prelude::*;

#[allow(unused_variables)]
#[component]
pub fn CollectionPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    // let tr: CollectionTranslate = translate(&lang);
    // let ctrl = Controller::new(lang, agit_id)?;
    // let mut show_filters = use_signal(|| false);

    // let collections = ctrl.collections();
    rsx! {}
    /* rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            // Main content
            div { class: "flex flex-col w-full h-full",
                h1 { class: "text-2xl sm:text-2xl font-bold font-Pretendard",
                    "{tr.collections} {agit_id}"
                }
                p { class: "text-sm  sm:text-sm text-gray-400", "1,120 Total Collections" }
                // Search and Filter Bar
                SearchFilterBar {
                    show_filter_btn: true,
                    on_filter_click: move |_| {
                        show_filters.toggle();
                    },
                    placeholder: tr.search_by_title,
                    on_add_click: move |_| {
                        ctrl.open_new_collection_modal();
                    },
                    // Fixme:
                    on_search_change: move |search_text| {},
                    on_search: move |search_text| {},
                    show_add_btn: true,
                    add_btn_text: tr.new_collection,
                }

                // Content area (FilterSidebar and Table)
                div { class: "flex flex-col md:flex-row flex-1 w-full",
                    // FilterSidebar (hidden on small screens unless toggled)
                    if *show_filters.read() {
                        div {
                            class: format!(
                                "w-64 bg-background border-r border-border-primary fixed inset-y-0 left-0 z-40 transform {} md:relative md:z-auto md:translate-x-0 transition-transform duration-300",
                                if *show_filters.read() { "translate-x-0" } else { "-translate-x-full" },
                            ),
                            FilterSidebar {
                                on_artist_change: move |artist| {},
                                value: "".to_string(),
                                attribute_value: "".to_string(),
                            }
                        }
                    }
                    // Table body
                    div { class: "flex-1 overflow-auto",
                        CollectionTable { lang: lang.clone(), agit_id: agit_id.clone() }
                    }
                }
            }
        }
    }*/
}
