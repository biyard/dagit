use bdk::prelude::*;

#[api_model(base = "/v1/artwork_ownerships", table = artwork_ownerships)]
pub struct ArtworkOwnership {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = artworks)]
    pub artwork_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
}
