use bdk::prelude::*;
use by_axum::auth::generate_jwt;
use by_types::Claims;
use common::{Result, error::ServiceError, tables::users::User};
use std::collections::HashMap;

pub struct AppClaims<'a>(pub &'a Claims);

impl<'a> AppClaims<'a> {
    // pub fn new(claims: &'a Claims) -> Self {
    //     Self(claims)
    // }

    pub fn generate_token(user: &User) -> Result<String> {
        let mut claims = Claims {
            sub: user.id.to_string(),
            role: by_types::Role::User,
            custom: HashMap::from([("email".to_string(), user.email.clone())]),
            ..Claims::default()
        };

        generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            ServiceError::JwtGenerationFailed(e.to_string())
        })
    }

    pub fn get_id(&self) -> i64 {
        self.0.sub.parse().unwrap_or_default()
    }
}
