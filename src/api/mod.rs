use axum::{
    routing::{get, post, put, delete},
    Router,
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};
use crate::database::mysql_orm;

pub mod user;

pub fn create_public_router() -> Router {
    Router::new()
        .route("/login", post(user::login))
        .route("/users", post(user::create_user))
}

pub fn create_private_router() -> Router {
    Router::new()
        .nest("/api", user::create_router())
}