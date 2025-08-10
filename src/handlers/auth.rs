use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    auth::{generate_jwt_token, hash_password, verify_password},
    models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse, User, RefreshTokenRequest},
    services::user_service,
    state::AppState,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let password_hash = hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::new_v4();
    let now = Utc::now();

    let user = User {
        id: user_id,
        name: payload.name,
        email: payload.email.clone(),
        role: "user".to_string(),
        password_hash,
        age: payload.age,
        created_at: now,
        updated_at: now,
    };

    user_service::create_user(&state.db, &user).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let refresh_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_refresh_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            age: user.age,
        },
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = user_service::get_user_by_email(&state.db, &payload.email).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let access_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let refresh_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_refresh_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            age: user.age,
        },
    }))
}

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let claims = crate::auth::verify_jwt_token(&payload.refresh_token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = user_service::get_user_by_id(&state.db, &claims.sub).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let access_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let refresh_token = generate_jwt_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
        state.config.jwt_refresh_expires_in,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            age: user.age,
        },
    }))
}