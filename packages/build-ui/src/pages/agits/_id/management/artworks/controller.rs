#![allow(unused)]
use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::{
    artworks::Artwork as ArtworkModel,
    prelude::{ArtworkByIdAction, ArtworkCreateRequest, ArtworkDeleteRequest, ArtworkQuery},
};

use crate::routes::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct ArtworkInputField {
    pub display_name: String,
    pub ways_to_sell: String,
    pub rarity: String,
    pub stock: String,
    pub price: String,
    pub lock_up_period: String,
    pub collection: String,
    pub medium: String,
    pub theme: String,
    pub art_style: String,
    pub material: String,
    pub color: String,
    pub size: String,
    pub weight: String,
    pub year: String,
    pub royalty: String,
    pub description: String,
    pub img_url: String,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    artworks: Signal<Vec<ArtworkModel>>,
    popup: PopupService,
    artwork_input_field: Signal<ArtworkInputField>,
}

impl Controller {
    pub fn new(lang: Language, agit_id: ReadOnlySignal<i64>) -> Result<Self, RenderError> {
        let mut popup: PopupService = use_context();
        let res = use_server_future(move || async move {
            let endpoint = crate::config::get().api_url;
            let client = ArtworkModel::get_client(endpoint);
            client
                .query(ArtworkQuery::new(100).with_page(0))
                .await
                .unwrap_or_default()
        })?;

        tracing::debug!("res: {:?}", res);
        let artwork_input_field = use_signal(|| ArtworkInputField {
            display_name: String::new(),
            ways_to_sell: String::new(),
            rarity: String::new(),
            stock: String::new(),
            price: String::new(),
            lock_up_period: String::new(),
            collection: String::new(),
            medium: String::new(),
            theme: String::new(),
            art_style: String::new(),
            material: String::new(),
            color: String::new(),
            size: String::new(),
            weight: String::new(),
            year: String::new(),
            royalty: String::new(),
            description: String::new(),
            img_url: String::new(),
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
            popup,
            artworks,
            artwork_input_field,
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn open_new_artwork_page(&self) {
        let navigate = use_navigator();

        if let Some(err) = navigate.push(Route::CreateArtworkPage {
            lang: self.lang,
            agit_id: self.agit_id.with(|id| *id),
        }) {
            tracing::error!("Navigation failed: {:?}", err);
        }
    }
    pub fn open_artwork_page(&self) {
        use_navigator().go_back();
    }

    pub fn update_artwork_field(&mut self, field: String, value: String) {
        match field.as_str() {
            "display_name" => self.artwork_input_field.set(ArtworkInputField {
                display_name: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "ways_to_sell" => self.artwork_input_field.set(ArtworkInputField {
                ways_to_sell: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "rarity" => self.artwork_input_field.set(ArtworkInputField {
                rarity: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "stock" => self.artwork_input_field.set(ArtworkInputField {
                stock: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "price" => self.artwork_input_field.set(ArtworkInputField {
                price: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "royalty" => self.artwork_input_field.set(ArtworkInputField {
                royalty: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "lock_up_period" => self.artwork_input_field.set(ArtworkInputField {
                lock_up_period: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "collection" => self.artwork_input_field.set(ArtworkInputField {
                collection: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "medium" => self.artwork_input_field.set(ArtworkInputField {
                medium: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "theme" => self.artwork_input_field.set(ArtworkInputField {
                theme: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "art_style" => self.artwork_input_field.set(ArtworkInputField {
                art_style: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "material" => self.artwork_input_field.set(ArtworkInputField {
                material: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "color" => self.artwork_input_field.set(ArtworkInputField {
                color: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "size" => self.artwork_input_field.set(ArtworkInputField {
                size: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "weight" => self.artwork_input_field.set(ArtworkInputField {
                weight: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "year" => self.artwork_input_field.set(ArtworkInputField {
                year: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            "description" => self.artwork_input_field.set(ArtworkInputField {
                description: value,
                ..self.artwork_input_field.with(|field| field.clone())
            }),
            _ => {
                btracing::error!("{} {}", self.lang, "invalid ...")
            }
        }
    }
}
