use crate::pages::agits::_id::management::collections::CollectionTranslate;
use bdk::prelude::*;
use common::tables::artworks::Artwork as ArtworkModel;
#[component]
pub fn NftTable(artworks: Vec<ArtworkModel>, lang: Language) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 align-items-start",

            {
                artworks
                    .iter()
                    .enumerate()
                    .map(|(index, artwork)| {
                        let dimensions = if index % 2 == 0 {
                            "width: 342px; height: 500px;"
                        } else {
                            "width: 342px; height: 350px;"
                        };
                        rsx! {
                            div {
                                key: "{artwork.id}",
                                class: "overflow-hidden hover:border-gray-600 transition-all duration-200 flex flex-col",
                                // Image container
                                div { class: "relative bg-gray-800", style: "{dimensions}",
                                    div { class: "text-4xl text-gray-600 absolute inset-0 flex items-center justify-center",
                                        img {
                                            class: "w-full h-full object-cover",
                                            src: artwork.art_image.clone(),
                                        }
                                    }
                                }
                                // NFT metadata section
                                div { class: "py-4 text-white flex-shrink-0",
                                    // Title and tag
                                    div { class: "flex items-center mb-2",
                                        span { class: "text-lg font-semibold", {artwork.title.clone()} }
                                        span { class: "ml-2 text-sm border border-gray-600 px-2 py-1", "TAG" }
                                    }
                                    // Artist name
                                    div { class: "text-sm text-gray-400 mb-2", "by {artwork.name}" }
                                    // Price, Sales Volume, and Owners
                                    div { class: "flex justify-between text-gray-400  text-sm",
                                        div { class: "flex flex-col",
                                            div { {tr.current_price} }
                                            span { class: "font-bold text-white", "${artwork.current_price}" }
                                        }
                                        div { class: "flex flex-col",
                                            div { {tr.sales_volume} }
                                            span { class: "font-bold text-white", "${artwork.volume_eth}" }
                                        }
                                        div { class: "flex flex-col",
                                            div { {tr.owner} }
                                            span { class: "font-bold text-white", "{artwork.owners}" }
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
