#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v1/artists", table = artists, action_by_id = [delete], iter_type = QueryResponse)]
pub struct Artist {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
}
