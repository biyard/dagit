#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use crate::v1::{artwork::Artwork, collection::Collection};

#[api_model(base = "/v1/agits", table = agits, action_by_id = [delete], iter_type = QueryResponse)]
pub struct Agit {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,

    #[api_model(summary, one_to_many = collections, foreign_key = agit_id)]
    pub collections: Vec<Collection>,
    #[api_model(summary, one_to_many = artworks, foreign_key = agit_id)]
    pub artworks: Vec<Artwork>,
}
