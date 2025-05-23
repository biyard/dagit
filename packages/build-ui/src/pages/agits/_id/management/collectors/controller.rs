#![allow(unused)]
use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::{
    collectors::Collector as CollectorModel,
    prelude::{
        CollectorByIdAction, CollectorCreateRequest, CollectorDeleteRequest, CollectorQuery,
    },
};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    collector: Signal<Vec<CollectorModel>>,
    popup: PopupService,
}
impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let popup: PopupService = use_context();

        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            client
                .query(CollectorQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;
        tracing::debug!("res: {:?}", res);

        let collector = use_signal(|| {
            (1..15)
                .map(|id| CollectorModel {
                    id,
                    total_volume: 2.370,
                    owned: 2,
                    token_ids: vec!["1234567890".to_string(), "1234567890".to_string()],
                    wallet_address: "0x1234567890abcdef".to_string(),
                    last_activity: "2023-10-01".to_string(),
                    verified: true,
                    created_at: chrono::Utc::now().timestamp(),
                    updated_at: chrono::Utc::now().timestamp(),
                    agit_id: id,
                    title: "".to_string(),
                    description: "".to_string(),
                    external_link: None,
                    banner_url: "".to_string(),
                    logo_url: "".to_string(),
                    authorized: false,
                    artworks: vec![],
                    holder: false,
                })
                .collect::<Vec<_>>()
        });

        let ctrl = Self {
            lang,
            collector,
            popup,
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
    pub fn create_collector(&self) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            let res = client
                .act(common::tables::prelude::CollectorAction::Create(
                    CollectorCreateRequest {
                        title: "".to_string(),
                        description: "".to_string(),
                        external_link: None,
                        banner_url: "".to_string(),
                        logo_url: "".to_string(),
                        verified: todo!(),
                        owned: todo!(),
                        token_ids: todo!(),
                        wallet_address: todo!(),
                        total_volume: todo!(),
                        last_activity: todo!(),
                    },
                ))
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collector created successfully");
                }
                Err(err) => {
                    tracing::error!("Error creating collector: {:?}", err);
                }
            }
        });
    }

    pub fn remove_collector(&self, collector_id: i64) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectorModel::get_client(endpoint);
            let res = client
                .act_by_id(
                    collector_id,
                    CollectorByIdAction::Delete(CollectorDeleteRequest {}),
                )
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collector removed successfully");
                }
                Err(err) => {
                    tracing::error!("Error removing collector: {:?}", err);
                }
            }
        });
    }
}
