use crate::components::table::{Table, TableHeaderCellProps};
use crate::pages::agits::components::PageTitle;

use super::controller::Controller;
use super::i18n::ArtworkTranslate;

use super::components::Header;

use bdk::prelude::*;
#[allow(unused_variables)]
#[component]
pub fn ArtworkPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let tr: ArtworkTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang, agit_id)?;
    rsx! {
        PageTitle { title: "Artworks", sub_title: format!("1,120 Total Artworks") }
        Header {
            lang,
            toggle_side_bar: move |_| {
                ctrl.toggle_side_bar();
            },
            on_view_change: move |v| {
                ctrl.set_view_mode(v);
            },
            on_filter_change: move |filter| {
                ctrl.set_filter(filter);
            },
            on_search: move |search| {
                ctrl.handle_search(search);
            },
            on_click_button: move |evt| {
                ctrl.handle_add_artwork();
            },
        }
        div { class: "w-full my-2.5 border border-neutral-90" }
        Table {
            columns: vec![
                TableHeaderCellProps {
                    label: tr.title.to_string(),
                    width: "100px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.attributes.to_string(),
                    width: "110px".to_string(),
                    sortable: false,
                },
                TableHeaderCellProps {
                    label: tr.ways_to_sell.to_string(),
                    width: "150px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.owner.to_string(),
                    width: "100px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.current_price.to_string(),
                    width: "150px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.average_price.to_string(),
                    width: "150px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.price_change.to_string(),
                    width: "150px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.volume.to_string(),
                    width: "150px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.royalty.to_string(),
                    width: "100px".to_string(),
                    sortable: true,
                },
                TableHeaderCellProps {
                    label: tr.status.to_string(),
                    width: "100px".to_string(),
                    sortable: true,
                },
            ],
            on_filter_change: move |v: String| {
                ctrl.handle_sort(&v);
            },
            tr {
                td { "AA" }
                td { "AA" }
            }
            tr {
                td { "B" }
                td { "B" }
            }
        }
    }
}
