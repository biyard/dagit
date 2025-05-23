use bdk::prelude::*;

// use crate::components::search_filter_bar::SearchFilterBar;
// use crate::components::tab_button::TabButton;
// use crate::pages::agits::_id::management::collections::controller::Controller;
// use crate::pages::agits::_id::management::collections::i18n::CollectionTranslate;

// use crate::pages::agits::_id::management::components::{ActivityTable, NftTable, OwnedTable};
// use crate::routes::Route;
// #[derive(Clone, Debug, PartialEq)]
// pub enum AssetTab {
//     List,
//     Activity,
// }

#[component]
#[allow(unused_variables)]
pub fn CollectionDetailPage(
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collection_id: i64,
) -> Element {
    // let search_query = use_signal(String::new);
    // let mut active_tab = use_signal(|| AssetTab::List);
    // let mut view_mode = use_signal(|| "table");
    // let tr: CollectionTranslate = translate(&lang);
    // let ctrl = Controller::new(lang, agit_id)?;
    // let artworks = ctrl.artworks();
    rsx! {}
    /* rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",

            div { class: "flex flex-col w-full h-full text-white",

                // Header with back button and collector info
                div { class: "flex flex-col mb-6",
                    div { class: "flex items-center",
                        Link {
                            to: Route::CollectionPage {
                                lang,
                                agit_id: agit_id(),
                            },
                            class: "text-gray-400 hover:text-white ",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M15 19l-7-7 7-7",
                                }
                            }
                        }


                        div { class: "flex items-center",

                            h1 { class: "text-xl font-bold",
                                {format!("{{Collection {}}}", collection_id)}
                            }
                        }
                    }
                    div { class: "text-sm text-gray-400 m-2",
                        "Last Activity 2mins ago / Joined Feb 01, 2025"
                    }
                }

                div { class: "mb-10",
                    div { class: "flex space-x-8 text-sm font-medium",
                        TabButton {
                            label: tr.list,
                            is_active: *active_tab.read() == AssetTab::List,
                            onclick: move |_| active_tab.set(AssetTab::List),
                        }


                        TabButton {
                            label: tr.activity,
                            is_active: *active_tab.read() == AssetTab::Activity,
                            onclick: move |_| active_tab.set(AssetTab::Activity),
                        }
                    }
                }


                SearchFilterBar {
                    show_filter_btn: true,
                    on_filter_click: move |_| {},
                    placeholder: tr.search_by_title,
                    on_add_click: move |_| {},
                    on_remove_click: move |_| {},
                    on_search_change: move |search_text| {},
                    on_search: move |search_text| {},
                    show_art_btn: true,
                    show_add_btn: true,
                    show_remove_btn: true,
                    show_all_filter_field: true,
                    on_view_mode_click: move |_| {
                        if *view_mode.read() == "table" {
                            view_mode.set("nftImages");
                        } else {
                            view_mode.set("table");
                        }
                    },
                    add_btn_text: tr.add_artwork,
                    remove_btn_text: tr.remove_artwork,
                }

                // Assets table
                div { class: "overflow-x-auto flex-1",


                    {
                        match active_tab.with(|tab| tab.clone()) {
                            AssetTab::List => {
                                if *view_mode.read() == "nftImages" {
                                    rsx! {
                                        NftTable { artworks, lang }
                                    }
                                } else {
                                    rsx! {
                                        OwnedTable { artworks, lang }
                                    }
                                }
                            }
                            AssetTab::Activity => {
                                if *view_mode.read() == "nftImages" {
                                    rsx! {
                                        NftTable { artworks, lang }
                                    }
                                } else {
                                    rsx! {
                                        ActivityTable { activity: artworks, lang }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }*/
}
