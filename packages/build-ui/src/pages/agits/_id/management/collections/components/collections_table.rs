use crate::pages::agits::_id::management::collections::{CollectionTranslate, Controller};
use crate::routes::Route;
use bdk::prelude::*;
use by_components::icons::{arrows, validations};

#[component]
pub fn CollectionTable(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::new(lang, agit_id)?;
    let tr: CollectionTranslate = translate(&lang);
    let collections = ctrl.collections();
    rsx! {
        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
            // Table header
            thead { class: "text-xs uppercase bg-table-background border-b border-border-primary",
                tr {
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { "#" }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.collection} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.floor_price} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.floor_change} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.volume_change} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.volume} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.owners} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.stock} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.status} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { "" }
                            validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                        }
                    } // For the actions column
                }
            }
            // Table body
            tbody {
                {
                    collections
                        .into_iter()
                        .enumerate()
                        .map(|(index, collection)| {
                            rsx! {
                                tr {
                                    key: "{index}",
                                    class: "border-b border-border-primary",
                                    onclick: move |_| {
                                        use_navigator()
                                            .push(Route::CollectionDetailPage {
                                                lang: lang,
                                                agit_id: *agit_id.read(),
                                                collection_id: collection.id as i64,
                                            });
                                    },
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.id}" }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "w-6 h-6 sm:w-8 sm:h-8 bg-border-primary mr-2" }
                                            span { "{collection.title}" }
                                            svg {
                                                view_box: "0 0 24 24",
                                                width: "16",
                                                height: "16",
                                                fill: "#10b981",
                                                class: "ml-1",
                                                path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{collection.floor_price_eth} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {collection.floor_price_eth}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{collection.floor_change_eth} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {collection.floor_change_eth}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center space-x-4 p-l-4",
                                            div { class: "flex flex-col",
                                                div { class: "text-green-500", "+ {collection.volume_change_24h}%" }
                                                div { class: "text-xs text-gray-400", "24h" }
                                            }
                                            div { class: "flex flex-col",
                                                div { class: "text-red-500", "{collection.volume_change_7d}%" }
                                                div { class: "text-xs text-gray-400", "7d" }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{collection.volume_eth} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {collection.volume_eth}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.owners}" }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.owners}" }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{collection.status}" }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        button { class: "text-gray-400 hover:text-white",
                                            validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                                        }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
