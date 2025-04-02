use axum::{
    Router,
    routing::post
};

use crate::database::mysql_orm;

pub mod user;

pub fn create_router() -> Router {
    Router::new()
        .nest("/api", user::create_router())
}