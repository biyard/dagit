use bdk::prelude::*;

use crate::tables::artworks::Artwork;

#[derive(validator::Validate)]
#[api_model(base = "/v1/collectors", table = collectors, action_by_id = [delete])]
pub struct Collector {
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
    #[api_model(summary, action=create, action_by_id = update)]
    pub verified: bool,
    #[api_model(summary, action=create action_by_id = update)]
    pub owned: i32,
    #[api_model(summary, action = create, action_by_id = update, type=JSONB)]
    pub token_ids: Vec<String>,
    #[api_model(summary, action = create, action_by_id = update)]
    pub wallet_address: String,
    #[api_model(summary, action=create, action_by_id = update)]
    pub total_volume: f64,
    #[api_model(summary, action=create, action_by_id = update)]
    pub last_activity: String,

    #[api_model(summary, one_to_many = artworks, foreign_key = collector_id)]
    pub artworks: Vec<Artwork>,

    #[api_model(summary, many_to_many = collector_holders, table_name = users, foreign_primary_key = user_id, foreign_reference_key = collector_id, aggregator = exist)]
    pub holder: bool,
}
