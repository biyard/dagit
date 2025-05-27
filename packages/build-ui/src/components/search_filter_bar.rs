use bdk::prelude::*;

use crate::components::{
    add_button::AddButton, all_filter_field::AllFilterField, art_button::ArtButton,
    filter_button::FilterButton, remove_button::RemoveButton, search_input::SearchInput,
};
#[component]
pub fn SearchFilterBar(
    #[props(default = false)] show_filter_btn: bool,
    #[props(default = None)] on_filter_click: Option<EventHandler<()>>,
    placeholder: String,
    #[props(default = None)] on_add_click: Option<EventHandler<()>>,
    #[props(default = None)] on_remove_click: Option<EventHandler<()>>,
    on_search_change: EventHandler<String>,
    on_search: EventHandler<String>,
    #[props(default = false)] show_add_btn: bool,
    #[props(default = false)] show_remove_btn: bool,
    #[props(default = None)] on_view_mode_click: Option<EventHandler<()>>,
    #[props(default = false)] show_art_btn: bool,
    #[props(default = false)] show_all_filter_field: bool,
    #[props(default = "".to_string())] add_btn_text: String,
    #[props(default = "".to_string())] remove_btn_text: String,
) -> Element {
    rsx! {
        div { class: "p-4 flex flex-col sm:flex-row sm:items-center gap-4",
            FilterButton { show: show_filter_btn, on_click: on_filter_click }
            ArtButton { show: show_art_btn, on_click: on_view_mode_click }
            AllFilterField { show: show_all_filter_field }
            SearchInput { placeholder, on_change: on_search_change }
            AddButton {
                show: show_add_btn,
                on_click: on_add_click,
                text: add_btn_text,
                remove_text: remove_btn_text.clone(),
            }
            RemoveButton {
                show: show_remove_btn,
                on_click: on_remove_click,
                text: remove_btn_text,
            }
        }
    }
}
