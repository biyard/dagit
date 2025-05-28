use bdk::prelude::{dioxus::CapturedError, *};
use common::{error::ServiceError, tables::prelude::Artwork};

use super::{components::tab_header::Tab, page::ItemDetailResult};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    active_tab: Signal<Tab>,
    title: Signal<Option<String>>,
    item_detail: Signal<Option<ItemDetailResult>>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let ctrl: Controller = Self {
            lang,
            agit_id,
            active_tab: use_signal(|| Tab::ItemDetails),
            item_detail: use_signal(|| None),
            title: use_signal(|| None),
        };
        tracing::debug!(
            "Controller created for CreateArtworkPage with agit_id: {} {}",
            ctrl.lang,
            ctrl.agit_id()
        );
        Ok(ctrl)
    }

    pub fn handle_go_back(&mut self) {
        tracing::debug!("Creating new artwork");
        let nav = use_navigator();
        nav.go_back();
    }

    pub fn set_active_tab(&mut self, tab: Tab) {
        self.active_tab.set(tab);
        // Here you would typically update the state to reflect the active tab
    }
    pub fn save_item_detail(&mut self, title: String, item_detail: ItemDetailResult) {
        self.title.set(Some(title));
        self.item_detail.set(Some(item_detail));
    }
    pub async fn create_artwork(
        &self,
        image_url: String,
        description: Option<String>,
    ) -> Result<Artwork, RenderError> {
        let agit_id = self.agit_id();
        if self.item_detail().is_none() {
            return Err(RenderError::Aborted(CapturedError::from_display(
                ServiceError::ValidationError("Item details are not set".into()),
            )));
        }
        let item_detail = self.item_detail().unwrap();
        let endpoint = crate::config::get().api_url;

        let client = Artwork::get_client(endpoint);

        let res = client
            .create(
                agit_id,
                self.title().clone().unwrap_or_default(),
                item_detail.ways_to_sell,
                item_detail.rarity,
                item_detail.stock,
                item_detail.royalty,
                item_detail.lockup_started_at,
                item_detail.lockup_ended_at,
                item_detail.medium,
                item_detail.theme,
                item_detail.art_style,
                item_detail.material,
                item_detail.color,
                item_detail.size,
                item_detail.weight,
                item_detail.year,
                image_url,
                description,
            )
            .await?;

        Ok(res)
    }
}
