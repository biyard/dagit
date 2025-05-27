use bdk::prelude::*;

use super::components::tab_header::Tab;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    active_tab: Signal<Tab>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let ctrl: Controller = Self {
            lang,
            agit_id,
            active_tab: use_signal(|| Tab::ItemDetails),
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
}
