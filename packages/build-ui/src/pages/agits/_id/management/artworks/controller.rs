#![allow(unused)]
use bdk::prelude::*;

use by_types::QueryResponse;
use dioxus_popup::PopupService;

use common::tables::prelude::{Artwork, ArtworkQuery, ArtworkSummary};

use crate::components::table::{SortConfig, SortDirection, TableHeaderCellProps};

use super::{
    components::{EditRowModal, EditRowModalTranslate},
    i18n::ArtworkTranslate,
};

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
#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    popup: PopupService,
    agit_id: ReadOnlySignal<i64>,
    view_mode: Signal<ViewMode>,

    columns: Signal<Vec<TableHeaderCellProps>>,

    filter: Signal<Filter>,
    show_side_bar: Signal<bool>,

    sort_config: Signal<Option<SortConfig>>,

    artworks: Resource<QueryResponse<ArtworkSummary>>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let popup: PopupService = use_context();
        let tr: ArtworkTranslate = translate(&lang);
        let artworks: Resource<QueryResponse<ArtworkSummary>> =
            use_server_future(move || async move {
                let endpoint = crate::config::get().api_url;
                let client = Artwork::get_client(endpoint);
                client
                    .query(agit_id(), ArtworkQuery::new(100).with_page(1))
                    .await
                    .unwrap_or_default()
            })?;
        use_effect(move || {
            let mut popup: PopupService = use_context();
            let tr: EditRowModalTranslate = translate(&lang);

            popup
                .open(rsx! {
                    EditRowModal {
                        lang,
                        on_close: move |_| {},
                        on_save: move |items| {},
                    }
                })
                .with_title(tr.title)
                .with_id("edit_row_modal");
        });
        let ctrl: Controller = Self {
            lang,
            popup,
            agit_id,
            view_mode: use_signal(|| ViewMode::Table),
            filter: use_signal(|| Filter::All),
            show_side_bar: use_signal(|| false),

            sort_config: use_signal(|| None),
            columns: use_signal(|| {
                vec![
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
                ]
            }),
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

    pub fn open_edit_row_modal(&mut self) {
        let tr: EditRowModalTranslate = translate(&self.lang);
        let mut popup = self.popup.clone();
        let ctrl = self.clone();
        popup
            .open(rsx! {
                EditRowModal {
                    lang: self.lang,
                    on_close: move |_| {
                        popup.close();
                    },
                    on_save: move |(sales_menu, attributes_menu)| {
                        tracing::debug!("Selected items: {:?}", sales_menu);
                        tracing::debug!("Selected attributes: {:?}", attributes_menu);
                        popup.close();
                    },
                }
            })
            .with_title(tr.title)
            .with_id("edit_row_modal");
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
