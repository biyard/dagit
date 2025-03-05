#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::v1::artwork::Artwork;
#[api_model(base = "/collections", table = collections)]
pub struct Collection {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,

    #[api_model(summary, one_to_many = artworks, foreign_key = collection_id)]
    pub artworks: Vec<Artwork>,
}
