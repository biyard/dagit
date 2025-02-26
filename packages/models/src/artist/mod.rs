#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/artists", table = artists)]
pub struct Artist {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action_by_id = update)]
    pub title: String,
}
