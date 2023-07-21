use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize)]
#[ts(export)]
pub struct User {
    pub name: String,
    pub role: String,
    pub age: i32,
    pub email: String,
}

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize)]
#[ts(export)]
pub struct Book {
    pub name: String,
    pub year: i32,
    pub writer: String,
}
