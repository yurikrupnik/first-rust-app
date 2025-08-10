use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use uuid::Uuid;

use crate::{
    models::{Claims, CreateUserRequest, UserResponse},
    services::user_service,
    state::AppState,
};

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = user_service::get_all_users(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            age: user.age,
        })
        .collect();

    Ok(Json(user_responses))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = user_service::get_user_by_id(&state.db, &id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        role: user.role,
        age: user.age,
    }))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let password_hash = crate::auth::hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user = crate::models::User {
        id: user_id,
        name: payload.name,
        email: payload.email,
        role: "user".to_string(),
        password_hash,
        age: payload.age,
        created_at: now,
        updated_at: now,
    };

    user_service::create_user(&state.db, &user).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        role: user.role,
        age: user.age,
    }))
}