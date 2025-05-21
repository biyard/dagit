use bdk::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ViewMode {
    Table,
    Gallery,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Translate)]
pub enum Filter {
    All,
    NFT,
    Physical,
}

impl From<&str> for Filter {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "all" => Filter::All,
            "nft" => Filter::NFT,
            "physical" => Filter::Physical,
            _ => Filter::All,
        }
    }
}
#[allow(unused)]
#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    view_mode: Signal<ViewMode>,
    filter: Signal<Filter>,
    show_side_bar: Signal<bool>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let ctrl: Controller = Self {
            lang,
            agit_id,
            view_mode: use_signal(|| ViewMode::Table),
            filter: use_signal(|| Filter::All),
            show_side_bar: use_signal(|| false),
        };

        Ok(ctrl)
    }

    pub fn toggle_side_bar(&mut self) {
        self.show_side_bar.toggle();
    }

    pub fn set_view_mode(&mut self, value: bool) {
        if value {
            self.view_mode.set(ViewMode::Gallery);
        } else {
            self.view_mode.set(ViewMode::Table);
        }
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter.set(filter);
    }

    pub fn handle_search(&mut self, search: String) {
        tracing::debug!("Searching for: {}", search);
        // Handle search logic here
    }

    pub fn handle_add_artwork(&mut self) {
        // Handle add artwork logic here
    }
}
