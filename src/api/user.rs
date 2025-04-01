use axum::{
    routing::{get, post, put, delete},
    Router,
    Json,
    extract::Path,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::database::mysql_orm::{self, Model as DbUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        Self {
            id: Some(db_user.id),
            name: db_user.name,
            email: db_user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
}

pub fn create_router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let db_user = mysql_orm::create_user(&db, payload.name, payload.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(User::from(db_user))))
}

async fn get_user(
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let db_user = mysql_orm::find_user_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(User::from(db_user)))
}

async fn update_user(
    Path(id): Path<i32>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let db_user = mysql_orm::update_user(&db, id, Some(payload.name), Some(payload.email))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(User::from(db_user)))
}

async fn delete_user(
    Path(id): Path<i32>,
) -> StatusCode {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    mysql_orm::delete_user(&db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    StatusCode::NO_CONTENT
}