use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum::response::Response;
use serde_json;
use validator::Validate;

use crate::{domain::url::Url, feature::url::service::{UrlService, UrlServiceTrait}};
use crate::feature::url::entity::CreateUrlDTO;

pub struct UrlHandler {
    url_service: Arc<UrlService>,
}

impl UrlHandler {
    pub fn new_handler(url_service: Arc<UrlService>) -> Self {
        Self { url_service }
    }
}

pub async fn get_all_url_handler_axum(
    State(handlers): State<Arc<UrlHandler>>,
) -> impl IntoResponse{
    match handlers.url_service.get_all_url().await {
        Ok(urls) => {
            let body = serde_json::to_string(&urls).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(body))
                .unwrap()
        },
        Err(_) => {
            let body = serde_json::to_string(&Vec::<Url>::new()).unwrap();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(body.into())
                .unwrap()
        }
    }
}



pub async fn create_url_handler(
    State(handlers): State<Arc<UrlHandler>>,
    Json(payload):Json<CreateUrlDTO>
) -> impl IntoResponse{
     if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(format!("Validation error: {:?}", validation_errors)),
        );
    }
  
    match handlers.url_service.create_url(payload.url).await {
         Ok(_) => (  StatusCode::CREATED,
            Json("Saved".to_string())
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Error while saving".to_string())
        )
    }
}