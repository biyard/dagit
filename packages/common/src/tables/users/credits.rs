use bdk::prelude::*;

#[api_model(base = "/v1/users/:user_id/credits", table = user_credits)]
pub struct UserCredit {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, action = add)]
    pub description: String,
    #[api_model(summary, action = add)]
    pub credit: i64,
}
