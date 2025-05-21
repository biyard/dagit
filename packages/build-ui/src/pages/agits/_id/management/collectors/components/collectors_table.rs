use crate::pages::agits::_id::management::collectors::controller::Controller;
use crate::pages::agits::_id::management::collectors::i18n::CollectorsTranslate;
use crate::routes::Route;
use bdk::prelude::*;
use by_components::icons::{arrows, validations};

#[component]
pub fn CollectorsTable(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let tr: CollectorsTranslate = translate(&lang);
    let ctrl = Controller::new(lang, agit_id)?;
    let collectors = ctrl.collector();
    rsx! {
        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
            thead { class: "text-xs text-white capitalize bg-table-background border-b border-border-primary",
                tr {
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.collector_id} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.total_volume} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.owned} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "Token ID" }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "Wallet Address" }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.last_activity} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-end",
                            span { "" }
                            validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                        }
                    }
                }
            }
            tbody {
                {
                    collectors
                        .into_iter()
                        .enumerate()
                        .map(|(_index, collector)| {
                            let collector_id = collector.id.clone();
                            rsx! {
                                tr {
                                    key: "{collector_id}",
                                    class: "hover:bg-gray-900 cursor-pointer",
                                    onclick: move |_| {
                                        let collector_id = collector.id.clone();
                                        use_navigator()
                                            .push(Route::CollectorDetailPage {
                                                lang: lang,
                                                agit_id: agit_id(),
                                                collector_id,
                                            });
                                    },
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "w-8 h-8 bg-white rounded-full mr-2 flex items-center justify-center" }
                                            div { class: "flex items-center",
                                                span { "10FEO!20" }
                                                span {
                                                    class: "ml-1 text-green-500 hidden aria-verified:block",
                                                    "aria-verified": collector.verified,
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
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { "{collector.total_volume} ETH" }
                                            span { class: "text-xs text-gray-500", "$ {collector.total_volume}" }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.owned.to_string()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex space-x-1",
                                            {
                                                collector
                                                    .token_ids
                                                    .iter()
                                                    .map(|token| {
                                                        rsx! {
                                                            span { class: "text-xs px-2 py-1 rounded border border-white", "{token}" }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.wallet_address.clone()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {collector.last_activity.clone()} }
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
