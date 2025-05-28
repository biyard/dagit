use bdk::prelude::*;

use by_types::QueryResponse;
use validator::Validate;

use crate::tables::{
    artists::Artist,
    prelude::{ArtworkOwnership, ArtworkPrice},
};

use super::{ArtStyle, Material, Medium, Rarity, Royalty, Size, Theme, WaysToSell, Weight};
#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Sorter {
    #[default]
    Title = 1,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Order {
    #[default]
    Asc = 1,
    Desc = 2,
}
#[derive(Validate)]
#[api_model(base = "/v1/agits/:agit_id/artworks", table = artworks, iter_type = QueryResponse, queryable = [(sort = Sorter, order = Order)])]
pub struct Artwork {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub title: String,

    // Sale Info
    #[api_model(summary, action = create)]
    pub ways_to_sell: WaysToSell,
    #[api_model(summary, action = create, nullable)]
    pub rarity: Option<Rarity>,
    #[api_model(summary, action = create)]
    pub stock: Option<i64>,
    #[api_model(summary, action = create, type = JSONB)]
    pub royalty: Royalty,
    #[api_model(summary, action = create, action_by_id = update, nullable)]
    pub lockup_started_at: Option<i64>,
    #[api_model(summary, action = create, action_by_id = update, nullable)]
    pub lockup_ended_at: Option<i64>,

    // Attributes
    #[api_model(summary, action = create, type = Integer)]
    pub medium: Medium,
    #[api_model(summary, action = create, type = JSONB)]
    pub theme: Theme,
    #[api_model(summary, action = create, type = JSONB)]
    pub art_style: Vec<ArtStyle>,
    #[api_model(summary, action = create, type = JSONB)]
    pub material: Vec<Material>,
    #[api_model(summary, action = create, type = JSONB)]
    pub color: Vec<String>,
    #[api_model(summary, action = create, type = JSONB)]
    pub size: Size,
    #[api_model(summary, action = create, type = JSONB)]
    pub weight: Weight,
    #[api_model(summary, action = create, type = JSONB)]
    pub year: i64,

    #[api_model(summary, skip, type = JSONB)]
    #[serde(default)]
    owner: ArtworkOwnership,

    #[api_model(summary, skip, type = JSONB)]
    pub price_last: ArtworkPrice,
    #[api_model(summary, skip, type = JSONB)]
    pub price_avg: ArtworkPrice,
    // Price change
    #[api_model(summary, skip, type = JSONB)]
    pub price_change_24h: ArtworkPrice,
    #[api_model(summary, skip, type = JSONB)]
    pub price_change_7d: ArtworkPrice,

    // Art Info
    #[api_model(summary, action = create)]
    pub image_url: String,

    #[api_model(action = create, nullable)]
    pub description: Option<String>,

    #[api_model(summary, action_by_id = update)]
    pub certified_at: Option<i64>,

    #[api_model(many_to_one = agits)]
    pub agit_id: i64,

    // Note: if collection_id is 0, it means the artwork is not in any collection
    #[api_model(skip)]
    pub collection_id: i64,

    #[api_model(summary, many_to_many = artwork_artist, table_name = artists, foreign_primary_key = artist_id, foreign_reference_key = artwork_id)]
    pub artist: Vec<Artist>,

    #[api_model(one_to_many = artwork_user_likes, foreign_key = artwork_id, aggregator = count)]
    pub likes: i64,
    #[api_model(many_to_many = artwork_user_likes, table_name = users, foreign_primary_key = user_id, foreign_reference_key = artwork_id, aggregator = exist)]
    pub liked: bool,
}
