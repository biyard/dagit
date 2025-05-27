use bdk::prelude::*;

#[api_model(base="/v1/categories", table = categories, action_by_id = [delete])]
pub struct Category {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, many_to_one = agits)]
    pub agit_id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
}
