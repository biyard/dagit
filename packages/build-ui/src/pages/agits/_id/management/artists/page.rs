// use crate::components::search_filter_bar::SearchFilterBar;
// use crate::routes::Route;
// use bdk::prelude::by_components::icons::{arrows, validations};
use bdk::prelude::*;

// use super::i18n::ArtistTranslate;

// use super::controller::Controller;
#[component]
#[allow(unused_variables)]
pub fn ArtistPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    // let ctrl = Controller::new(lang, agit_id)?;
    // let tr: ArtistTranslate = translate(&lang);
    // let view_mode = use_signal(|| "table");
    // let search_query = use_signal(String::new);
    // let artists = ctrl.artist();
    rsx! {}
    /* rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            div { class: "flex flex-col w-full h-full",
                div { class: "flex flex-col mb-6",
                    h1 { class: "text-2xl font-bold", {tr.artists} }
                    p { class: "text-sm text-gray-400", "1,201 Total Artists" }
                }

                SearchFilterBar {
                    show_filter_btn: true,
                    on_filter_click: move |_| {},
                    on_search_change: move |search_text| {},
                    on_search: move |search_text| {},
                    show_all_filter_field: true,
                    placeholder: tr.search_by_title,
                    show_add_btn: true,
                    on_add_click: move |_| {
                        ctrl.open_new_artist_form();
                    },
                    add_btn_text: tr.new_artist,
                    remove_btn_text: tr.remove_artist,
                }


                div { class: " flex-1 overflow-x-auto",
                    table { class: "w-full text-sm text-left border-collapse min-w-[800px]",
                        thead { class: "text-xs text-white capitalize bg-table-background border-b border-border-primary",
                            tr {

                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.artist} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.revenue} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.artworks} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    div { class: "flex items-center",
                                        span { {tr.featured_work} }
                                        arrows::UpDown {
                                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                                            height: 18,
                                            width: 18,
                                        }
                                    }
                                }
                                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                    "Attributes"
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
                                        span { {tr.social_media} }
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
                                        validations::Extra {
                                            class: "[&>circle]:stroke-white",
                                            height: 18,
                                        }
                                    }
                                }
                            }
                        }
                        tbody {
                            {
                                artists
                                    .into_iter()
                                    .enumerate()
                                    .map(|(_index, artist)| {
                                        let artist_id = artist.id.clone();
                                        rsx! {
                                            tr {
                                                key: "owned-tr-{artist_id}",
                                                class: "hover:bg-gray-900 cursor-pointer",
                                                onclick: move |_| {
                                                    let artist_id = artist.id.clone();
                                                    use_navigator()
                                                        .push(Route::ArtistDetailPage {
                                                            lang: lang,
                                                            agit_id: agit_id(),
                                                            artist_id,
                                                        });
                                                },
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex items-center",
                                                        div { class: "w-8 h-8 bg-white rounded-full mr-2 flex items-center justify-center" }
                                                        div { class: "flex items-center",
                                                            div { class: "flex flex-col",
                                                                span { {artist.name.clone()} }
                                                                span { {artist.mail.clone()} }
                                                            }
                                                        }
                                                    }
                                                }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex flex-col",
                                                        span { "{artist.revenue} ETH" }
                                                        span { class: "text-xs text-gray-500", "${artist.revenue}" }
                                                    }
                                                }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artist.artworks.to_string()} }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artist.featured_work.to_string()} }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                                    div { class: "flex space-x-1",
                                                        {
                                                            artist
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
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artist.status.clone()} }
                                                td { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap", {artist.social_media.clone()} }
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
        }
    }
    */
}
