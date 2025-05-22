use bdk::prelude::{by_types::QueryResponse, *};
use common::tables::prelude::{Artwork, ArtworkQuery, ArtworkSummary};

use crate::components::table::{SortConfig, SortDirection};

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

    sort_config: Signal<Option<SortConfig>>,

    artworks: Resource<QueryResponse<ArtworkSummary>>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let artworks: Resource<QueryResponse<ArtworkSummary>> =
            use_server_future(move || async move {
                let endpoint = crate::config::get().api_url;
                let client = Artwork::get_client(endpoint);
                client
                    .query(ArtworkQuery::new(100).with_page(1))
                    .await
                    .unwrap_or_default()
            })?;

        let ctrl: Controller = Self {
            lang,
            agit_id,
            view_mode: use_signal(|| ViewMode::Table),
            filter: use_signal(|| Filter::All),
            show_side_bar: use_signal(|| false),

            sort_config: use_signal(|| None),
            artworks,
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

    pub fn handle_sort(&mut self, key: &str) {
        let current_sort_config = self.sort_config();
        match current_sort_config {
            Some(sort_config) if sort_config.key == key => {
                self.sort_config.set(Some(SortConfig {
                    key: key.to_string(),
                    direction: match sort_config.direction {
                        SortDirection::Asc => SortDirection::Desc,
                        SortDirection::Desc => SortDirection::Asc,
                    },
                }));
            }
            _ => {
                self.sort_config.set(Some(SortConfig {
                    key: key.to_string(),
                    direction: SortDirection::Asc,
                }));
            }
        }
    }
}
