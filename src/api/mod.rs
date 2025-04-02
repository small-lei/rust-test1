use axum::{
    Router,
    routing::post
};

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