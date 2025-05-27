pub mod config;
pub mod controllers;

use bdk::prelude::*;
use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::{Router, middleware},
};

use controllers::v1;

use by_types::DatabaseConfig;
use common::tables::{
    agit_admins::AgitAdmin,
    agits::Agit,
    artists::Artist,
    artworks::Artwork,
    categories::Category,
    collections::Collection,
    collectors::Collector,
    topics::Topic,
    users::{User, UserCredit},
};
use common::{Result, tables::user_terms::UserTerm};
use sqlx::{migrate, postgres::PgPoolOptions};
use tokio::net::TcpListener;
mod utils;

macro_rules! migrate {
    ($pool:ident, $($table:ident),* $(,)?) => {
        {
            $(
                let t = $table::get_repository($pool.clone());
                tracing::debug!("Creating table: {:?}", stringify!($table));
                t.create_this_table().await?;
            )*
            $(
                let t = $table::get_repository($pool.clone());
                tracing::debug!("Creating Relative: {:?}", stringify!($table));
                t.create_related_tables().await?;
            )*
        }
    };
}
async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    //TODO: Add Model Migration
    tracing::info!("Running migration");
    migrate!(
        pool, User, UserCredit, Artist, Agit, Collection, Artwork, AgitAdmin, UserTerm, Category,
        Topic, Collector
    );
    tracing::info!("Migration done");

    Ok(())
}
async fn make_app() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    set_auth_config(conf.auth.clone());

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let app = app
        .nest("/v1", v1::routes(pool.clone())?)
        .layer(middleware::from_fn(authorization_middleware));

    Ok(app)
}
#[tokio::main]
async fn main() -> Result<()> {
    let app = make_app().await?;
    let port = config::get().port;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
pub mod dagit_tests {
    use by_types::Claims;
    use common::error::ServiceError;
    use common::tables::users::User;
    use rest_api::ApiService;
    use std::{collections::HashMap, time::SystemTime};

    use super::*;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub now: i64,
        pub id: String,
        pub claims: Claims,
        pub endpoint: String,
        pub user: User,
        pub user_token: String,
    }
    const IMAGE_URL: &str =
        "https://metadata.dev.dagit.club/images/72a11429-20c0-4d62-8cde-ff3d4d5dc0bb";

    pub async fn setup_test_user(id: &str, pool: &sqlx::PgPool) -> Result<User> {
        let user = User::get_repository(pool.clone());
        let agit = Agit::get_repository(pool.clone());
        let agit_admins = AgitAdmin::get_repository(pool.clone());
        let email = format!("user-{id}@test.com");
        let address = format!("test-user-address-{id}");
        let name = format!("test-user-{id}");
        let profile_url = None::<String>;
        let mut tx = pool.begin().await?;

        let user = user
            .insert_with_tx(
                &mut *tx,
                common::tables::users::AuthProvider::Google,
                address,
                email,
                name,
                profile_url,
            )
            .await?
            .ok_or(ServiceError::Unknown("Create Test User Failed".to_string()))?;

        let agit = agit
            .insert_with_tx(
                &mut *tx,
                format!("test-agit-{id}"),
                format!("description"),
                None,
                IMAGE_URL.to_string(),
                IMAGE_URL.to_string(),
                false,
            )
            .await?
            .ok_or(ServiceError::Unknown("Create Agit Failed".to_string()))?;

        agit_admins
            .insert_with_tx(&mut *tx, agit.id, user.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert agit admin: {:?}", e);
                ServiceError::Unknown("Create Agit Admin Failed".to_string())
            })?;

        let user = User::query_builder()
            .id_equals(user.id)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ServiceError::DuplicateUser)?;
        tx.commit().await?;

        Ok(user)
    }

    pub fn setup_jwt_token(user: User) -> (Claims, String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims {
            sub: user.id.to_string(),
            exp: now + 3600,
            role: by_types::Role::User,
            custom: HashMap::new(),
        };
        let token = by_axum::auth::generate_jwt(&mut claims).unwrap();
        (claims, token)
    }

    pub async fn setup() -> Result<TestContext> {
        if option_env!("JWT_SECRET_KEY").is_none() {
            unsafe {
                std::env::set_var("JWT_SECRET_KEY", "default_test_secret_key");
            }
        }

        let conf = config::get();
        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at := EXTRACT(EPOCH FROM now()) * 1000;
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_created_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.created_at := EXTRACT(EPOCH FROM now()) * 1000;
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let app = make_app().await?;
        let app = by_axum::into_api_adapter(app);

        let id = uuid::Uuid::new_v4().to_string();
        tracing::debug!("id: {id}");
        let user = setup_test_user(&id, &pool).await?;
        tracing::debug!("user: {:?}", user);

        let (claims, user_token) = setup_jwt_token(user.clone());

        let app = Box::new(app);
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", user_token));

        Ok(TestContext {
            pool,
            app,
            user,
            id: id.to_string(),
            user_token,
            claims,
            now: now as i64,
            endpoint: "http://localhost:3000".to_string(),
        })
    }
}
