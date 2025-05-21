#![allow(unused)]
use super::i18n::{
    CollectionNameInputModalTranslate, NewCollectionModalTranslate, SuccessModalTranslate,
    TransferConfirmationModalTranslate,
};
use crate::config::Config;
use crate::pages::agits::_id::management::collections::components::{
    CollectionNameInputModal, NewCollectionModal, SuccessModal, TransferConfirmationModal,
};
use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::artworks;
use common::tables::{
    artists::Artist as ArtistModel,
    artworks::Artwork as ArtworkModel,
    collections::Collection as CollectionModel,
    prelude::{
        ArtistByIdAction, ArtistCreateRequest, ArtistDeleteRequest, ArtistQuery,
        CollectionByIdAction, CollectionCreateRequest, CollectionDeleteRequest, CollectionQuery,
    },
};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collections: Signal<Vec<CollectionModel>>,
    artworks: Signal<Vec<ArtworkModel>>,
    selected_artworks: Signal<Vec<usize>>,
    collection_name: Signal<String>,

    popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectionModel::get_client(endpoint);
            client
                .query(CollectionQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;

        let collections = use_signal(|| {
            (1..15)
                .map(|id| CollectionModel {
                    id,
                    created_at: chrono::Utc::now().timestamp(),
                    updated_at: chrono::Utc::now().timestamp(),
                    agit_id: 1,
                    title: "(Collection Name)".to_string(),
                    description: "Description".to_string(),
                    external_link: None,
                    banner_url: "".to_string(),
                    logo_url: "".to_string(),
                    authorized: true,
                    artworks: vec![],
                    likes: 0,
                    followers: 0,
                    liked: false,
                    followed: false,
                    holder: true,
                    custodian: true,
                    verified: true,
                    floor_price_eth: 2.370,
                    floor_change_eth: 2.370,
                    volume_change_24h: 12.0,
                    volume_change_7d: -8.0,
                    volume_eth: 2.370,
                    owners: "Num".to_string(),
                    status: "Active".to_string(),
                    stock: "Num".to_string(),
                })
                .collect::<Vec<_>>()
        });

        let artworks = use_signal(|| {
            (0..4)
                .map(|id| ArtworkModel {
                    id,
                    title: "(Art Title)".to_string(),
                    name: "Artist Name".to_string(),
                    verified: true,
                    collection_type: Some("Happy".to_string()),
                    attributes_type: vec!["Paid".to_string(), "Verified".to_string()],
                    ways_to_sell: "Bid".to_string(),
                    volume_eth: 2.370,
                    volume_usd: 8147.63,
                    current_price: 2.370,
                    average_price: 2.370,
                    royalty: 2.370,
                    price_change: 12.0,
                    owners: 145,
                    status: "Active".to_string(),
                    created_at: chrono::Utc::now().timestamp(),
                    updated_at: chrono::Utc::now().timestamp(),
                    external_link: None,
                    description: "Description".to_string(),
                    agit_id: 1,
                    collection_id: Some((1)),
                    artist_id:1,
                    owner_id:1,
                    likes: 0,
                    liked: false,
                    art_image: "https://res.cloudinary.com/dgesrup3u/image/upload/v1744880242/Screenshot_2025-04-17_at_9.56.47_AM_ll2cwy.png".to_string(),
                    last_price:100,
                    medium: "Digital".to_string(),
                    rarity: "Rare".to_string(),
                    activity_id: "1".to_string(),
                    activity_from: "20114FWO".to_string(),
                    activity_to: "20114FWO".to_string(),
                    activity_time: "30 mins ago".to_string(),
                    activity_title: "Art Title".to_string(),
                })
                .collect::<Vec<_>>()
        });

        let selected_artworks = use_signal(|| Vec::<usize>::new());
        let collection_name = use_signal(|| String::new());

        let ctrl = Self {
            lang,
            agit_id,
            popup,
            artworks,
            collections,
            selected_artworks,
            collection_name,
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn create_collection(&self) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectionModel::get_client(endpoint);
            let res = client
                .act(common::tables::prelude::CollectionAction::Create(
                    CollectionCreateRequest {
                        title: "".to_string(),
                        description: "".to_string(),
                        external_link: None,
                        banner_url: "".to_string(),
                        logo_url: "".to_string(),
                        verified: false,
                        floor_price_eth: 0.0,
                        floor_change_eth: 0.0,
                        volume_eth: 0.0,
                        volume_change_24h: 0.0,
                        volume_change_7d: 0.0,
                        owners: "".to_string(),
                        status: "".to_string(),
                        stock: "".to_string(),
                    },
                ))
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collection created successfully");
                }
                Err(err) => {
                    tracing::error!("Error creating collection: {:?}", err);
                }
            }
        });
    }

    pub fn remove_collection(&self, collection_id: i64) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = CollectionModel::get_client(endpoint);
            let res = client
                .act_by_id(
                    collection_id,
                    CollectionByIdAction::Delete(CollectionDeleteRequest {}),
                )
                .await;
            match res {
                Ok(_) => {
                    tracing::debug!("Collection removed successfully");
                }
                Err(err) => {
                    tracing::error!("Error removing collection: {:?}", err);
                }
            }
        });
    }

    #[allow(dead_code)]
    pub fn open_new_collection_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: NewCollectionModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        let artworks_data = self.artworks.read().clone();
        let mut selected_artworks = self.selected_artworks.clone();
        popup
            .open(rsx! {
                NewCollectionModal {
                    lang: self.lang,
                    on_close: move |_| {
                        popup.close();
                    },
                    on_select_artworks: move |selected: Vec<usize>| {
                        selected_artworks.set(selected.clone());
                        ctrl.open_transfer_confimation_modal();
                    },
                    artworks: artworks_data,
                }
            })
            .with_id("new-collection-modal")
            .with_title(tr.title);
    }

    #[allow(dead_code)]
    pub fn open_transfer_confimation_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: TransferConfirmationModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        let selected_count = self.selected_artworks.read().len();
        popup
            .open(rsx! {
                TransferConfirmationModal {
                    lang: self.lang,
                    selected_count,
                    on_back: move |_| {
                        popup.close();
                    },
                    on_continue: move |_| {
                        ctrl.open_collection_name_input_modal();
                    },
                }
            })
            .with_id("transfer-confirmation-modal")
            .with_title(tr.title);
    }

    #[allow(dead_code)]
    pub fn open_collection_name_input_modal(&self) {
        let mut popup = self.popup.clone();
        let mut collection_name = self.collection_name.clone();
        let ctrl = self.clone();
        let tr: CollectionNameInputModalTranslate = translate(&self.lang);
        popup
            .open(rsx! {
                CollectionNameInputModal {
                    lang: self.lang,
                    on_back: move |_| {
                        popup.close();
                    },
                    on_add: move |name: String| {
                        collection_name.set(name.clone());
                        ctrl.open_success_modal();
                    },
                }
            })
            .with_id("collection-name-modal")
            .with_title(tr.title);
    }

    #[allow(dead_code)]
    pub fn open_success_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: SuccessModalTranslate = translate(&self.lang);
        let collection_name = self.collection_name.read().clone();
        popup
            .open(rsx! {
                SuccessModal {
                    lang: self.lang,
                    collection_name,
                    on_confirm: move |_| {
                        popup.close();
                    },
                }
            })
            .with_id("success-modal")
            .with_title(tr.title);
    }
}
