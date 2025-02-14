use crate::api::middleware::json::CustomJson;
use crate::api::request::login::LoginRequest;
use crate::api::response::error::{AppError, Status};
use crate::api::response::login::LoginResponse;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use anyhow::anyhow;
use argon2::Argon2;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    match entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&payload.username))
        .all(state.db_conn.load().as_ref())
        .await
    {
        Ok(admins) => {
            if admins.is_empty() {
                return Err(AppError(
                    StatusCode::UNAUTHORIZED,
                    anyhow!("User is not an admin"),
                ));
            }

            let admin = &admins[0];
            if validate_password(&payload.password, &admin.password).is_err() {
                return Err(AppError(
                    StatusCode::UNAUTHORIZED,
                    anyhow!("Password mismatch"),
                ));
            }
        }
        Err(e) => return Err(AppError(StatusCode::UNAUTHORIZED, e.into())),
    }

    let secret = &state.settings.load().token_secret;
    let timeout = state.settings.load().token_timeout_seconds;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(timeout)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or("".to_string());

    let response = LoginResponse {
        status: Status::Success,
        token,
    };

    Ok(Json(response))
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))
}

