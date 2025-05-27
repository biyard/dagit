use bdk::prelude::{by_components::icons::validations, *};

use crate::{
    components::{
        button::{PrimaryButton, SecondaryButton},
        checkbox::Checkbox,
        search_filter_bar::SearchFilterBar,
    },
    pages::agits::_id::management::collections::i18n::NewCollectionModalTranslate,
};

use by_components::icons::arrows;
use common::tables::prelude::Artwork as ArtworkModel;

#[component]
#[allow(unused_variables)]
pub fn NewCollectionModal(
    on_close: EventHandler<()>,
    artworks: Vec<ArtworkModel>,
    on_select_artworks: EventHandler<Vec<usize>>,
    lang: Language,
) -> Element {
    // Use `use_signal` for a Vec<usize> to store selected artwork IDs
    let mut selected_artworks = use_signal(|| Vec::<usize>::new());
    let tr: NewCollectionModalTranslate = translate(&lang);

    rsx! {

        div { class: " flex flex-col",

            // Modal header
            div { class: "border-b border-border-primary",
                p { class: "text-sm text-gray-400 mt-1", {tr.sub_title} }
            }

            // Search and filters
            SearchFilterBar {
                show_filter_btn: true,
                on_filter_click: move |_| {},
                placeholder: tr.search_placeholder,
                on_search_change: move |value| {},
                on_search: move |value| {},
                show_all_filter_field: true,
            }

            // Table
            div { class: "flex-1 overflow-auto",
                table { class: "w-full text-sm text-left",
                    thead { class: "text-xs uppercase border-b border-border-primary",
                        tr {
                            th { class: "px-4 py-3 w-8",
                                div {
                                    validations::Check {
                                        class: "border border-neutral-80",
                                        height: 18,
                                        width: 18,
                                    }
                                }
                            }
                            th { class: "px-4 py-3",
                                div { class: "flex items-center ",
                                    span { class: "text-white", "Title" }
                                    arrows::UpDown {
                                        class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                        height: 18,
                                        width: 18,
                                    }
                                }
                            }
                            th { class: "px-4 py-3 text-white", "Collection" }
                            th { class: "px-4 py-3 text-white", "Attributes" }
                            th { class: "px-4 py-3",
                                div { class: "flex items-center",
                                    span { class: "text-white", "Ways to Sell" }
                                    arrows::UpDown {
                                        class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                        height: 18,
                                        width: 18,
                                    }
                                }
                            }
                            th { class: "px-4 py-3",
                                div { class: "flex items-center",
                                    span { class: "text-white", "Volume" }
                                    arrows::UpDown {
                                        class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                        height: 18,
                                        width: 18,
                                    }
                                }
                            }
                            th { class: "px-4 py-3",
                                div { class: "flex items-center",
                                    span { class: "text-white", "Status" }
                                    arrows::UpDown {
                                        class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                        height: 18,
                                        width: 18,
                                    }
                                }
                            }
                            th { class: "px-4 py-3", "" }
                        }
                    }
                    tbody {
                        // Iterate over artworks using .read()
                        {
                            artworks
                                .into_iter()
                                .enumerate()
                                .map(move |(index, artwork)| {
                                    rsx! {
                                        tr { class: "border-b border-border-primary", key: "{index}",
                                            td { class: "px-4 py-3",
                                                Checkbox {
                                                    id: format!("checkbox_{}", artwork.id),
                                                    checked: selected_artworks.read().contains(&(artwork.id as usize)),
                                                    onchange: move |checked| {
                                                        selected_artworks
                                                            .with_mut(|vec| {
                                                                if checked {
                                                                    vec.push(artwork.id as usize);
                                                                } else {
                                                                    vec.retain(|&x| x != (artwork.id as usize));
                                                                }
                                                            });
                                                    },
                                                }
                                            }
                                            td { class: "px-4 py-3",
                                                div { class: "flex items-center",
                                                    div { class: "w-8 h-8 bg-border-primary mr-2" }
                                                    div { class: "flex flex-col",
                                                        div { class: "flex items-center text-popup-text",
                                                            span { "{artwork.title}" }
                                                            svg {
                                                                view_box: "0 0 24 24",
                                                                width: "16",
                                                                height: "16",
                                                                fill: "#10b981",
                                                                class: "ml-1",
                                                                path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                                            }
                                                        }
                                                        div { class: "text-sm text-gray-400", "{artwork.name}" }
                                                    }
                                                }
                                            }
                                            td { class: "px-4 py-3 text-popup-text",
                                                "{artwork.collection_type.as_ref().unwrap_or(&String::new())}"
                                            }
                                            td { class: "px-4 py-3",
                                                div { class: "flex gap-2",
                                                    {
                                                        artwork
                                                            .attributes_type
                                                            .iter()
                                                            .map(|attr| {
                                                                rsx! {
                                                                    span { class: "px-2 py-1 bg-transparent border border-border-primary text-xs rounded text-popup-text",
                                                                        "{attr}"
                                                                    }
                                                                }
                                                            })
                                                    }
                                                }
                                            }
                                            td { class: "px-4 py-3 text-popup-text", "{artwork.ways_to_sell}" }
                                            td { class: "px-4 py-3",
                                                div { class: "text-popup-text", "{artwork.volume_eth} ETH" }
                                                div { class: "text-xs text-gray-400", "$ {artwork.volume_usd}" }
                                            }
                                            td { class: "px-4 py-3 text-popup-text ", "{artwork.status}" }
                                            td { class: "px-4 py-3",
                                                validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                                            }
                                        }
                                    }
                                })
                        }
                    }
                }
            }

            // Footer
            div { class: "p-4 border-t border-border-primary flex flex-col justify-end self-end",
                div { class: "text-sm text-gray-400 mb-5",
                    "{selected_artworks.read().len()} {tr.artwork_selected}"
                }
                div { class: "flex gap-4",
                    PrimaryButton { onclick: move |_| on_close.call(()), disabled: false, {tr.cancel_btn_txt} }

                    SecondaryButton {
                        disabled: selected_artworks.read().is_empty(),
                        onclick: move |_| on_select_artworks.call(selected_artworks.read().clone()),
                        {tr.confirm_btn_txt}
                    }
                }
            }
        }
    }
}
