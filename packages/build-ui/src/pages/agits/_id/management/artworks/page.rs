use crate::components::table::Table;
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
            columns: ctrl.columns(),
            on_filter_change: move |v: String| {
                ctrl.handle_sort(&v);
            },
            for artwork in ctrl.artworks()?.items {
                tr {
                    td { {artwork.title} }
                }
            }
        }
    }
}
