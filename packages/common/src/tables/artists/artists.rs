use bdk::prelude::*;

#[api_model(base = "/v1/artists", table = artists, action_by_id = [delete], iter_type = by_types::QueryResponse)]
pub struct Artist {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = [update])]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub mail: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub social_media: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub intro: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub biography: String,

    #[api_model(summary, action = create, action_by_id = [update])]
    pub revenue: f64,
    #[api_model(summary, action = create, action_by_id = [update], type=JSONB)]
    pub attributes_type: Vec<String>,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub featured_work: String,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub artworks: i64,
    #[api_model(summary, action = create, action_by_id = [update])]
    pub status: String,
}
