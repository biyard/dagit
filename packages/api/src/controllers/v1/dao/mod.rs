    #![allow(unused)]
    use bdk::prelude::*;
    use by_axum::{
        aide,
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, Query, State},
            routing::{get, post},
        },
    };
    use common::Result;
    use common::tables::prelude::*;

    #[derive(
        Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
    )]
    pub struct DaoPathParam {
        pub id: i64,
    }

    #[derive(Clone, Debug)]
    pub struct DaoController {
        pool: sqlx::PgPool,
        repo: Dao,
    }