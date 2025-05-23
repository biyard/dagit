#![allow(unused)]
use super::{
    i18n::{
        ConfirmRemoveArtistModalTranslate, RemovalSuccessModalTranslate,
        RemoveArtistNameModalTranslate,
    },
    models::*,
};
use common::tables::{
    artists::Artist as ArtistModel,
    artworks::Artwork as ArtworkModel,
    prelude::{ArtistByIdAction, ArtistCreateRequest, ArtistDeleteRequest, ArtistQuery},
};
use wasm_bindgen_futures::spawn_local;

use bdk::prelude::{dioxus_popup::PopupService, *};

use crate::{
    config::Config,
    pages::agits::_id::management::artists::components::{
        ConfirmRemoveArtistModal, RemovalSuccessModal, RemoveArtistModal,
    },
    routes::Route,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artist: Signal<Vec<ArtistModel>>,
    artworks: Signal<Vec<ArtworkModel>>,
    artist_input_field: Signal<ArtistInputField>,
    popup: PopupService,
}
impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            client
                .query(ArtistQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;
        tracing::debug!("res: {:?}", res);
        let artist_input_field = use_signal(|| ArtistInputField {
            display_name: String::new(),
            social_media: String::new(),
            medium: String::new(),
            theme: String::new(),
            art_style: String::new(),
            introduction: String::new(),
            biography: String::new(),
        });
        let artist = use_signal(|| {
            (1..10)
                .map(|id| ArtistModel {
                    id,
                    name: "Artist Name".to_string(),
                    mail: "email@email.com".to_string(),
                    revenue: 2.370,
                    attributes_type: vec![
                        "Pixel".to_string(),
                        "Animation".to_string(),
                        "Sci-fi".to_string(),
                        "3D".to_string(),
                        "Digital".to_string(),
                    ],
                    status: "true".to_string(),
                    social_media: "@social_media".to_string(),
                    featured_work: "Artwork_title".to_string(),
                    created_at: chrono::Utc::now().timestamp(),
                    updated_at: chrono::Utc::now().timestamp(),
                    title: "Artist Title".to_string(),
                    intro: "".to_string(),
                    biography: "".to_string(),
                    artworks: 247,
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

        let ctrl = Self {
            lang,
            agit_id,
            artist,
            artworks,
            artist_input_field,
            popup,
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn create_artist(&self) {
        let artist_inputs = self.artist_input_field.with(|field| field.clone());
        // act_by_id is with id, update or delete.
        // act is without id. Create
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            let res = client
                .act(common::tables::prelude::ArtistAction::Create(
                    ArtistCreateRequest {
                        title: artist_inputs.display_name,
                        mail: artist_inputs.social_media.clone(),
                        social_media: artist_inputs.medium.clone(),
                        intro: artist_inputs.theme,
                        biography: artist_inputs.art_style,
                        revenue: todo!(),
                        attributes_type: todo!(),
                        featured_work: todo!(),
                        artworks: todo!(),
                        name: todo!(),
                        status: todo!(),
                    },
                ))
                .await;
            tracing::debug!("mail: {:?}", artist_inputs.social_media);
            tracing::debug!("social_media: {:?}", artist_inputs.medium);
            match res {
                Ok(_) => {
                    btracing::info!("Artist created successfully");
                }
                Err(e) => {
                    btracing::error!("Error creating artist: {:?}", e);
                }
            };
        });
    }
    pub fn remove_artist(&self, artist_id: i64) {
        spawn_local(async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtistModel::get_client(endpoint);
            let res = client
                .act_by_id(
                    artist_id,
                    common::tables::prelude::ArtistByIdAction::Delete(ArtistDeleteRequest {}),
                )
                .await;
            match res {
                Ok(_) => {
                    btracing::info!("Artist removed successfully");
                }
                Err(e) => {
                    btracing::error!("Error removing artist: {:?}", e);
                }
            };
        });
    }

    pub fn update_artist_field(&mut self, field: String, value: String) {
        match field.as_str() {
            "display_name" => self.artist_input_field.set(ArtistInputField {
                display_name: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            "social_media" => self.artist_input_field.set(ArtistInputField {
                social_media: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            "medium" => self.artist_input_field.set(ArtistInputField {
                medium: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            "theme" => self.artist_input_field.set(ArtistInputField {
                theme: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            "introduction" => self.artist_input_field.set(ArtistInputField {
                introduction: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),

            "biography" => self.artist_input_field.set(ArtistInputField {
                biography: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            "art_style" => self.artist_input_field.set(ArtistInputField {
                art_style: value,
                ..self.artist_input_field.with(|field| field.clone())
            }),
            _ => {
                btracing::error!("{} {}", self.lang, "invalid ...")
            }
        }
    }

    pub fn open_new_artist_form(&self) {
        let navigate = use_navigator();
        navigate.push(Route::CreateArtistPage {
            lang: self.lang,
            agit_id: self.agit_id.with(|id| *id),
        });
    }
    pub fn open_edit_artist_form(&self, artist_id: i64) {
        let navigate = use_navigator();
        navigate.push(Route::EditArtistPage {
            lang: self.lang,
            agit_id: self.agit_id(),
            artist_id,
        });
    }

    #[allow(dead_code)]
    pub fn confirm_removal_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: ConfirmRemoveArtistModalTranslate = translate(&self.lang);
        let ctrl = self.clone();

        popup
            .open(rsx!(
                ConfirmRemoveArtistModal {
                    on_back: move |_| {
                        popup.close();
                    },
                    on_remove: move |_| {
                        ctrl.confirm_name_removal_modal();
                    },
                    lang: self.lang,
                }
            ))
            .with_id("remove-artist-modal")
            .with_title(tr.title);
    }
    #[allow(dead_code)]
    pub fn confirm_name_removal_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: RemoveArtistNameModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        popup
            .open(rsx!(
                RemoveArtistModal {
                    on_back: move |_| {
                        popup.close();
                    },
                    on_remove: move |_| {
                        ctrl.success_modal();
                    },
                    lang: self.lang,
                }
            ))
            .with_id("remove-artistName-modal")
            .with_title(tr.title);
    }
    #[allow(dead_code)]
    pub fn success_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: RemovalSuccessModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        popup
            .open(rsx!(
                RemovalSuccessModal {
                    on_back: move |_| {},
                    on_confirm: move |_| {},
                    lang: self.lang,
                }
            ))
            .with_id("remove-artist-modal-success")
            .with_title(tr.title);
    }

    pub fn go_back(&self) {
        use_navigator().go_back();
    }
}
