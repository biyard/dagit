use crate::pages::agits::_id::management::{
    collections::CollectionTranslate, components::TableHeader,
};
use bdk::prelude::{by_components::icons::validations, *};
use common::tables::artworks::Artwork as ArtworkModel;
#[component]
pub fn OwnedTable(artworks: Vec<ArtworkModel>, lang: Language) -> Element {
    let mut active_dropdown = use_signal(|| None::<usize>);

    rsx! {
        table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
            TableHeader { lang }
            tbody {
                {
                    artworks
                        .iter()
                        .enumerate()
                        .map(|(index, artwork)| {
                            let is_dropdown_open = active_dropdown
                                .with(|active| active == &Some(index));
                            rsx! {
                                tr {
                                    key: "owned-tr-{index}",
                                    class: "border-b border-gray-800 hover:bg-gray-900",
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex items-center",
                                            div { class: "w-8 h-8 bg-white rounded mr-2 flex items-center justify-center",
                                                ""
                                            }
                                            div { class: "flex flex-col",
                                                div { class: "flex items-center",
                                                    span { {artwork.title.clone()} }
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
                                                span { class: "text-xs text-gray-500", {artwork.name.clone()} }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex space-x-1",
                                            {
                                                artwork
                                                    .attributes_type
                                                    .iter()
                                                    .map(|attr| {
                                                        rsx! {
                                                            span { class: "text-xs px-2 py-1 rounded border border-white", {attr.clone()} }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artwork.ways_to_sell.clone()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artwork.owners.to_string()} }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {artwork.current_price.to_string()} }
                                            span { class: "text-xs text-gray-500", {artwork.current_price.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {artwork.average_price.to_string()} }
                                            span { class: "text-xs text-gray-500", {artwork.average_price.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex justify-between",
                                            div { class: "flex flex-col",
                                                span { class: "text-green-500", "+{artwork.price_change.to_string()}%" }
                                                span { class: "pl-4 text-xs", "24h" }
                                            }
                                            div { class: "flex flex-col",
                                                span { class: "text-red-500", {artwork.price_change.to_string()} }
                                                span { class: "pr-4 text-xs", "7d" }
                                            }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {artwork.volume_eth.to_string()} }
                                            span { class: "text-xs text-gray-500", {artwork.volume_usd.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                        div { class: "flex flex-col",
                                            span { {artwork.royalty.to_string()} }
                                            span { class: "text-xs text-gray-500", {artwork.royalty.to_string()} }
                                        }
                                    }
                                    td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artwork.status.to_string()} }
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

#[component]
#[allow(unused_variables)]
pub fn ActivityTable(activity: Vec<ArtworkModel>, lang: Language) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    rsx! {
        {
            activity
                .iter()
                .enumerate()
                .map(|(index, activity)| {
                    rsx! {
                        div { class: "flex items-center p-4 mb-2.5 rounded bg-table-bg hover:bg-hover-activity transition-colors duration-200 group",
                            // Art thumbnail
                            div { class: "w-16 h-16 bg-[#f0c14b] mr-4 rounded flex justify-center items-center" }
                            // Content
                            div { class: "flex-1",
                                // Title row
                                div { class: "flex items-center mb-1",
                                    span { class: "font-bold text-base mr-1", "Art Title" }
                                    // Verified icon
                                    svg {
                                        view_box: "0 0 24 24",
                                        width: "20",
                                        height: "20",
                                        fill: "#10b981",
                                        class: "ml-1",
                                        path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
                                    }
                                }
                                // Purchase info
                                div { class: "flex items-center mb-1 text-sm text-gray-400",
                                    span { "Purchased by " }
                                    // User avatar
                                    div { class: "w-4 h-4 bg-white rounded-full mx-1 inline-block" }
                                    span { class: "mr-1 text-white", "20114FWO" }
                                    span { class: "text-gray-400", "from" }
                                    // User avatar
                                    div { class: "w-4 h-4 bg-white rounded-full mx-1 inline-block" }
                                    span { class: "text-white", "20114FWO" }
                                }
                                // Timestamp
                                div { class: "text-sm text-gray-400 flex items-center",
                                    span { "30 mins ago" }
                                    // External link icon
                                    span { class: "ml-1 text-xs", "↗" }
                                }
                            }
                        }
                    }
                })
        }
    }
}
