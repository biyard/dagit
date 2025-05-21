use crate::pages::agits::_id::management::artworks::{
    controller::Controller, i18n::ArtworkTranslate,
};
use bdk::prelude::*;
use by_components::icons::{arrows, validations};

#[component]
pub fn ArtworkTable(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let tr: ArtworkTranslate = translate(&lang);
    let ctrl = Controller::new(lang, agit_id)?;
    let artworks = ctrl.artworks();
    rsx! {
        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
            // Table header
            thead { class: "text-xs uppercase bg-table-background border-b border-border-primary",
                tr {
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.title} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.attributes} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.ways_to_sell} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.owner} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.current_price} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.average_price} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.price_change} }
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
                            span { {tr.royalty} }
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
                    artworks
                        .into_iter()
                        .enumerate()
                        .map(|(index, artwork)| {
                            rsx! {
                                tr {
                                    key: "{index}",
                                    class: "border-b border-border-primary",
                                    onclick: move |_| {},
                                    // Table row content...
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "w-6 h-6 sm:w-8 sm:h-8 bg-white mr-2" }
                                            span { "{artwork.title}" }
                                            // Verified icon
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
                                        div { class: "flex space-x-1",
                                            {
                                                artwork
                                                    .attributes_type
                                                    .iter()
                                                    .map(|attribute| {
                                                        rsx! {
                                                            span { class: "text-xs px-2 py-1 rounded border border-white", "{attribute}" }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{artwork.ways_to_sell}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{artwork.owners}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{artwork.current_price} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {artwork.current_price}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{artwork.average_price} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {artwork.average_price}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center space-x-4 p-l-4",
                                            div { class: "flex flex-col",
                                                div { class: "text-green-500", "+ {artwork.price_change}%" }
                                                div { class: "text-xs text-gray-400", "24h" }
                                            }
                                            div { class: "flex flex-col",
                                                div { class: "text-red-500", "{artwork.price_change}%" }
                                                div { class: "text-xs text-gray-400", "7d" }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { "{artwork.volume_eth} ETH" }
                                        div { class: "text-xs text-gray-400", "$ {artwork.volume_eth}" }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{artwork.royalty}" }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", "{artwork.status}" }
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
