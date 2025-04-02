use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
    Json,
    Extension
};
use serde::{Deserialize, Serialize};
use crate::database::mysql_orm::{self, Model as DbUser};
use crate::middleware::auth_middleware;

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
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    code: u16,
    message: String,
    data: Option<T>,
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, Json<ApiResponse<()>>> {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|e| {
            Json(ApiResponse {
                code: 500,
                message: format!("数据库连接失败: {}", e),
                data: None,
            })
        })?;

    let user = mysql_orm::find_user_by_email(&db, &payload.email)
        .await
        .map_err(|e| {
            Json(ApiResponse {
                code: 500,
                message: format!("用户查询失败: {}", e),
                data: None,
            })
        })?
        .ok_or(Json(ApiResponse {
            code: 404,
            message: "用户不存在".to_string(),
            data: None,
        }))?;

    // if !mysql_orm::verify_password(&user.password, &payload.password) {
    //     return Err(Json(ApiResponse {
    //         code: 401,
    //         message: "密码错误".to_string(),
    //         data: None,
    //     }));
    // }

    let token = crate::middleware::auth::generate_token(user.id)
        .map_err(|_| Json(ApiResponse {
            code: 500,
            message: "Token生成失败".to_string(),
            data: None,
        }))?;

    Ok(Json(ApiResponse {
        code: 200,
        message: "登录成功".to_string(),
        data: Some(token),
    }))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/users/:id", get(get_user).route_layer(axum::middleware::from_fn(auth_middleware)))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let db = mysql_orm::establish_connection()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let db_user = mysql_orm::create_user(&db, payload.name, payload.email, payload.password)
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