use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize, Debug)]
#[ts(export)]
pub struct Book {
    pub name: String,
    pub year: i32,
    pub writer: String,
}