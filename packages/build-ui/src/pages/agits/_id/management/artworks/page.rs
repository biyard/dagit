use crate::components::artwork_grid::ArtworkGrid;
use crate::components::table::{DropDownOptionProps, Table};
use crate::pages::agits::_id::management::artworks::controller::ViewMode;
use crate::pages::agits::components::PageTitle;

use super::controller::Controller;

use super::components::{filter::FilterSidebar, header::Header};

use bdk::prelude::*;
#[component]
pub fn ArtworkPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, agit_id)?;
    rsx! {
        PageTitle {
            title: "Artworks",
            sub_title: format!("{} Total Artworks", ctrl.artworks()?.total_count),
        }
        Header {
            lang,
            toggle_side_bar: move |_| {
                ctrl.toggle_side_bar();
            },
            toggle_view_mode: move |_| {
                ctrl.toggle_view_mode();
            },
            on_filter_change: move |filter| {
                ctrl.set_filter(filter);
            },
            on_search: move |search| {
                ctrl.handle_search(search);
            },
            on_click_button: move |_evt| {
                ctrl.handle_add_artwork();
            },
        }
        div { class: "w-full my-2.5 border border-neutral-90" }
        div { class: "flex flex-row gap-2.5 flex-1",
            div {
                aria_hidden: !ctrl.show_side_bar(),
                class: "aria-hidden:hidden",
                FilterSidebar { class: "shrink-0" }
            }
            div {
                class: "flex-1 aria-hidden:hidden",
                aria_hidden: ctrl.view_mode() != ViewMode::Gallery,
                ArtworkGrid {
                    onclick: move |id| {
                        ctrl.handle_artwork_click(id);
                    },
                    items: ctrl.artworks()?.items.into_iter().map(|v| v.into()).collect(),
                
                }
            }
            div {
                class: "aria-hidden:hidden flex-1 overflow-x-scroll",
                aria_hidden: ctrl.view_mode() != ViewMode::Table,
                Table {
                    columns: ctrl.columns(),
                    on_filter_change: move |v: String| {
                        ctrl.handle_sort(&v);
                    },
                    dropdown_options: vec![
                        DropDownOptionProps {
                            label: "Edit Top Row Items".to_string(),
                            onclick: Callback::new(move |_| {
                                ctrl.open_edit_row_modal();
                            }),
                        },
                    ],
                    for artwork in ctrl.artworks()?.items {
                        tr {
                            td { {artwork.title} }
                        }
                    }
                }
            }
        }
    }
}
