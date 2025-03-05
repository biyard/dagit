#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use super::artwork::Artwork;

#[api_model(base = "/v1/agits/:agit-id/collections", table = collections, action_by_id = [delete], iter_type = QueryResponse)]
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

    #[api_model(summary, one_to_many = artworks, foreign_key = collection_id)]
    pub artworks: Vec<Artwork>,
}
