use axum::{http::StatusCode, response::{IntoResponse, Response}, Json, body::Bytes, extract::Request, middleware::Next};
use tracing::{debug, trace, info};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // 用户ID
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub message: String,
}

// JWT配置结构
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

impl JwtConfig {
    pub fn new() -> Self {
        JwtConfig {
            secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string()),
            expiration: 3600, // 1小时
        }
    }
}

// 生成token
pub fn generate_token(user_id: i32) -> Result<String, AuthError> {
    let config = JwtConfig::new();
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(config.expiration))
        .expect("invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
    .map_err(|_| AuthError {
        message: "生成token失败".to_string(),
    })
}

// 验证中间件
pub async fn auth_middleware(request: Request, next: Next) -> Result<Response, AuthError> {
    let token = request
        .headers()
        .get("Authorization")
        .map(|header| {
            println!("发现Authorization请求头: {:?}", header);
            header
        })
        .and_then(|header| header.to_str().ok())
        // .and_then(|header| {
        //     header.strip_prefix("Bearer ")
        //         .or_else(|| header.strip_prefix("Bearer"))
        //         .filter(|s| !s.is_empty())
        //         .map(str::trim)
        // })
        .ok_or_else(|| {
            println!("无效的Bearer格式或空令牌");
            AuthError {
                message: "无效的认证头".to_string(),
            }
        })?;

    let config = JwtConfig::new();
    let validation = Validation::default();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &validation,
    )
    .map_err(|_| AuthError {
        message: "无效的token".to_string(),
    })?;

    Ok(next.run(request).await)
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNAUTHORIZED;
        let body = Json(self);
        (status, body).into_response()
    }
}