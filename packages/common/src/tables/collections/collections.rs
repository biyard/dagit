use bdk::prelude::*;

use crate::tables::artworks::Artwork;

//TODO(api): Implement "/likes", "/followers" and "/holder" APIs
//TODO(api): Implement admin api "m1/collections/:id" for managing custodian.
#[derive(validator::Validate)]
#[api_model(base = "/v1/collections", table = collections, action_by_id = [delete])]
#[serde(default)]
pub struct Collection {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, action = create, action_by_id = update, nullable)]
    pub external_link: Option<String>,
    #[api_model(summary, action = create, action_by_id = update)]
    #[validate(url)]
    pub banner_url: String,
    #[api_model(summary, action = create, action_by_id = update)]
    #[validate(url)]
    pub logo_url: String,
    #[api_model(summary, action_by_id = update)]
    pub authorized: bool,
    #[api_model(summary, action = create, action_by_id = update)]
    pub verified: bool,
    #[api_model(summary, action = create, action_by_id = update)]
    pub floor_price_eth: f64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub floor_change_eth: f64,
    #[api_model(summary,action = create, action_by_id = update)]
    pub volume_change_24h: f64,
    #[api_model(summary,action = create, action_by_id = update)]
    pub volume_change_7d: f64,
    #[api_model(summary,action = create, action_by_id = update)]
    pub volume_eth: f64,
    #[api_model(summary,action = create, action_by_id = update)]
    pub owners: String,
    #[api_model(summary, action = create,action_by_id = update)]
    pub status: String,
    #[api_model(summary, action = create,action_by_id = update)]
    pub stock: String,

    #[api_model(summary, one_to_many = artworks, foreign_key = collection_id)]
    #[serde(default)]
    pub artworks: Vec<Artwork>,

    #[api_model(summary, one_to_many = collection_user_likes, foreign_key = collection_id, aggregator = count)]
    pub likes: i64,

    #[api_model(summary, one_to_many = collection_user_followers, foreign_key = collection_id, aggregator = count)]
    pub followers: i64,

    #[api_model(summary, many_to_many = collection_user_likes, table_name = users, foreign_primary_key = user_id, foreign_reference_key = collection_id, aggregator = exist)]
    pub liked: bool,
    #[api_model(summary, many_to_many = collection_user_followers, table_name = users, foreign_primary_key = user_id, foreign_reference_key = collection_id, aggregator = exist)]
    pub followed: bool,
    #[api_model(summary, many_to_many = collection_holders, table_name = users, foreign_primary_key = user_id, foreign_reference_key = collection_id, aggregator = exist)]
    pub holder: bool,
    #[api_model(summary, many_to_many = collection_custodians, table_name = users, foreign_primary_key = user_id, foreign_reference_key = collection_id, aggregator = exist)]
    pub custodian: bool,
}
