use by_axum::{aide::axum::IntoApiResponse, axum::{http::StatusCode, routing::{get, post}, Router}};
use by_axum::axum::{extract::{Json, Path}, response::IntoResponse, Extension};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use models::v1::agit::{AgitRepository, AgitQuery, AgitReadAction, AgitRepositoryUpdateRequest};

#[derive(Deserialize, Serialize)]
pub struct CreateAgit {
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateAgit {
    pub title: String,
}

pub async fn create_agit(
    Json(payload): Json<CreateAgit>,
    Extension(repo): Extension<Arc<AgitRepository>>,
) -> impl IntoApiResponse {
    match repo.insert(payload.title).await {
        Ok(agit) => (StatusCode::CREATED, Json(agit)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to create agit: {}", e))).into_response(),
    }
}

pub async fn get_agits(
    Extension(repo): Extension<Arc<AgitRepository>>,
) -> impl IntoApiResponse {
    let query = AgitQuery { 
        size: 10, 
        bookmark: None, 
    };

    match repo.find(&query).await {
        Ok(agits) => (StatusCode::OK, Json(agits)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to retrieve agits: {}", e))).into_response(),
    }
}

pub async fn get_agit_by_id(
    Path(id): Path<i64>,
    Extension(repo): Extension<Arc<AgitRepository>>,
) -> impl IntoApiResponse {
    let query = AgitReadAction {
        // id: id, #TODO
    };

    match repo.find_one(&query).await {
        Ok(agit) => (StatusCode::OK, Json(agit)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to retrieve agit: {}", e))).into_response(),
    }
}

pub async fn update_agit(
    Path(id): Path<i64>,
    Json(payload): Json<UpdateAgit>,
    Extension(repo): Extension<Arc<AgitRepository>>,
) -> impl IntoResponse {    
    let update_request = AgitRepositoryUpdateRequest {
        title: Some(payload.title),
    };

    match repo.update(id, update_request).await {
        Ok(updated_agit) => (StatusCode::OK, Json(updated_agit)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to update agit: {}", e))).into_response(),
    }
}

pub async fn delete_agit(
    Path(id): Path<i64>,
    Extension(repo): Extension<Arc<AgitRepository>>,
) -> impl IntoApiResponse {
    match repo.delete(id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to delete agit: {}", e))).into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/v1/agits", post(create_agit)) 
        .route("/v1/agits", get(get_agits))  
        .route("/v1/agits/:id", get(get_agit_by_id))  
        .route("/v1/agits/update/:id", post(update_agit))  
        .route("/v1/agits/delete/:id", post(delete_agit)) 
}
