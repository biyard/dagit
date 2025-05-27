use bdk::prelude::*;

use crate::tables::agits::Agit;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", table = users, read_action = refresh, action = [signup(agit_title = String, terms_agreed_at = i64, ads_agreed_at = Option<i64>)])]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = [signup, login], type = INTEGER)]
    pub provider: AuthProvider,
    #[api_model(unique, action = [signup, login], read_action = get_user_by_address)]
    pub address: String,

    #[api_model(action = [signup, update_profile])]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, update_profile])]
    pub name: String,

    #[api_model(action = [signup, update_profile], nullable)]
    #[validate(url)]
    pub profile_url: Option<String>,

    #[api_model(one_to_many = user_credits, foreign_key = user_id, aggregator = sum(credit))]
    pub credits: i64,

    #[api_model(many_to_many = agit_admins, foreign_table_name = agits, foreign_primary_key = agit_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub agits: Vec<Agit>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AuthProvider {
    #[default]
    Google = 1,
}
