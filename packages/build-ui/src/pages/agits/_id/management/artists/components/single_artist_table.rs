use crate::pages::agits::_id::management::artists::controller::Controller;
use crate::pages::agits::_id::management::artists::i18n::ArtistTranslate;
use bdk::prelude::by_components::icons::arrows;
use bdk::prelude::{by_components::icons::validations, *};
use common::tables::artworks::Artwork as ArtworkModel;
#[component]
pub fn SingleArtistTable(
    assets: Vec<ArtworkModel>,
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artist_id: i64,
) -> Element {
    let mut active_dropdown = use_signal(|| None::<usize>);
    let ctrl = Controller::new(lang, agit_id)?;
    let tr: ArtistTranslate = translate(&lang);

    rsx! {
        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
            thead {
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
                            span { {tr.medium} }
                            arrows::UpDown {
                                class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                height: 18,
                                width: 18,
                            }
                        }
                    }
                    th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        div { class: "flex items-center",
                            span { {tr.rarity} }
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
                            span { {tr.sales_volume} }
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
                    }
                }
            }
            tbody {
                {
                    assets
                        .into_iter()
                        .enumerate()
                        .map(|(index, asset)| {
                            let is_dropdown_open = active_dropdown
                                .with(|active| active == &Some(index));
                            rsx! {
                                tr {
                                    key: "owned-tr-{index}",
                                    class: "border-b border-gray-800 hover:bg-gray-900",
                                    onclick: move |_| ctrl.open_edit_artist_form(index as i64),
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "w-8 h-8 bg-white rounded mr-2 flex items-center justify-center",
                                                ""
                                            }
                                            div { class: "flex flex-col",
                                                div { class: "flex items-center",
                                                    span { {asset.title.clone()} }
                                                    span { class: "ml-1 text-green-500",
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
                                                span { class: "text-xs text-gray-500", {asset.name.clone()} }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {asset.medium.clone()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {asset.rarity.clone()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {asset.owners.to_string()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {asset.current_price.to_string()} }
                                            span { class: "text-xs text-gray-500", {asset.current_price.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {asset.average_price.to_string()} }
                                            span { class: "text-xs text-gray-500", {asset.average_price.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex justify-between",
                                            div { class: "flex flex-col",
                                                span { class: "", "+{asset.price_change.to_string()}%" }
                                                span { class: "pl-4 text-xs", "24h" }
                                            }
                                            div { class: "flex flex-col",
                                                span { class: "", {asset.price_change.to_string()} }
                                                span { class: "pr-4 text-xs", "7d" }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {asset.volume_eth.to_string()} }
                                            span { class: "text-xs text-gray-500", {asset.volume_usd.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {asset.status.to_string()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "relative",
                                            button {
                                                class: "text-gray-400 hover:text-white",
                                                onclick: move |_| {
                                                    active_dropdown.set(if is_dropdown_open { None } else { Some(index) });
                                                },
                                                validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                                            }
                                            div {
                                                class: "absolute right-0 mt-2 w-48 bg-black border border-green-500 rounded shadow-lg z-10 hidden aria-dropdown-open:block",
                                                "aria-dropdown-open": is_dropdown_open,
                                                div { class: "py-1",
                                                    button {
                                                        class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                        onclick: move |_| {
                                                            tracing::debug!("Edit the artwork clicked");
                                                            active_dropdown.set(None);
                                                        },
                                                        "Edit the artwork"
                                                    }
                                                    button {
                                                        class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                        onclick: move |_| {
                                                            tracing::debug!("Remove from the list clicked");
                                                            active_dropdown.set(None);
                                                        },
                                                        "Remove from the list"
                                                    }
                                                    button {
                                                        class: "block w-full text-left px-4 py-2 text-sm text-white hover:bg-gray-800",
                                                        onclick: move |_| {
                                                            tracing::debug!("Option clicked");
                                                            active_dropdown.set(None);
                                                        },
                                                        "Option"
                                                    }
                                                }
                                            }
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
