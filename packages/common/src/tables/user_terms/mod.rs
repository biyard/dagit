use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(base = "/v1/user-terms", table = user_terms)]
pub struct UserTerm {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(action = [insert])]
    pub terms_agreed_at: i64,
    #[api_model(action = [insert], nullable)]
    pub ads_agreed_at: Option<i64>,

    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
}
