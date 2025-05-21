use bdk::prelude::{by_components::icons::validations::Add, *};

use by_components::icons::{layouts::Window, settings::Sliders};

use super::super::controller::Filter;
use crate::components::{
    button::{IconButton, SecondaryButton},
    dropdown::DropDown,
    search_input::SearchInput,
};

translate! {
    ArtworkPageHeaderTranslate;

    placeholder: {
        en: "Search by title",
        ko: "제목 검색",
    },

    new_artwork: {
        en: "New Artwork",
        ko: "작품 추가",
    }
}

#[component]
pub fn Header(
    lang: Language,
    toggle_side_bar: EventHandler<bool>,
    on_view_change: EventHandler<bool>,
    on_filter_change: EventHandler<Filter>,
    on_search: EventHandler<String>,
    on_click_button: EventHandler<MouseEvent>,
) -> Element {
    let tr: ArtworkPageHeaderTranslate = translate(&lang);
    let options = Filter::VARIANTS
        .iter()
        .map(|v| v.translate(&lang).to_string())
        .collect::<Vec<_>>();
    rsx! {
        div { class: "flex gap-3 w-full",
            IconButton { onclick: move |_| {}, Sliders {} }
            IconButton { onclick: move |_| {}, Window {} }
            DropDown {
                class: "min-w-[200px]",
                default_value: Filter::All.translate(&lang).to_string(),
                options,
                on_change: move |evt: String| {
                    let filter = Filter::from(evt.as_str());
                    on_filter_change.call(filter);
                },
            }
            SearchInput {
                class: "flex-1",
                placeholder: tr.placeholder,
                on_change: move |evt: String| {
                    on_search.call(evt);
                },
            }
            SecondaryButton { onclick: on_click_button,
                div { class: "flex gap-1 text-white",
                    Add { class: "[&>path]:stroke-white" }
                    {tr.new_artwork}
                }
            }
        
        }
    }
}
