#![allow(unused)]
use crate::tables::{artworks::Artwork, categories::Category, collections::Collection};

use bdk::prelude::*;
use validator::Validate;

// #[cfg(feature = "server")]
// use bdk::prelude::by_types::QueryResponse;

//TODO(api): Implement "/likes", "/followers" and "/member" APIs
//TODO(api): Implement admin api "m1/agit/:id" for managing admins.
#[derive(Validate)]
#[api_model(base = "/v1/agits", table = agits, action_by_id = [delete], iter_type = by_types::QueryResponse)]
#[serde(default)]
pub struct Agit {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, action = create, action_by_id = update, nullable)]
    pub external_link: Option<String>,
    #[api_model(summary, action = create, action_by_id = update)]
    #[validate(url)]
    pub banner_url: String,
    #[api_model(summary, action = create, action_by_id = update)]
    #[validate(url)]
    pub logo_url: String,
    #[api_model(summary, action_by_id = update)]
    #[serde(default)]
    pub authorized: bool,

    #[api_model(summary, one_to_many = collections, foreign_key = agit_id)]
    #[serde(default)]
    pub collections: Vec<Collection>,

    #[api_model(summary, one_to_many = artworks, foreign_key = agit_id)]
    #[serde(default)]
    pub artworks: Vec<Artwork>,

    #[api_model(summary, one_to_many=categories, foreign_key=agit_id)]
    #[serde(default)]
    pub categories: Vec<Category>,

    #[api_model(summary, one_to_many = agit_user_likes, foreign_key = agit_id, aggregator = count)]
    pub likes: i64,
    #[api_model(summary, many_to_many = agit_user_likes, table_name = users, foreign_primary_key = user_id, foreign_reference_key = agit_id, aggregator = exist)]
    pub liked: bool,

    #[api_model(summary, one_to_many = agit_user_followers, foreign_key = agit_id, aggregator = count)]
    pub followers: i64,
    #[api_model(summary, many_to_many = agit_user_followers, table_name = users, foreign_primary_key = user_id, foreign_reference_key = agit_id, aggregator = exist)]
    pub followed: bool,

    #[api_model(summary, many_to_many = agit_members, table_name = users, foreign_primary_key = user_id, foreign_reference_key = agit_id, aggregator = exist)]
    pub member: bool,
    #[api_model(summary, many_to_many = agit_admins, table_name = users, foreign_primary_key = user_id, foreign_reference_key = agit_id, aggregator = exist)]
    pub admin: bool,
}
