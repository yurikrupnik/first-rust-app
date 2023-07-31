use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize, Debug)]
#[ts(export)]
pub struct User {
    pub name: String,
    pub role: String,
    pub age: i32,
    pub email: String,
    // pub provider: String,
    // #[serde(rename = "tenantId")]
    // pub tenant_id: String,
    // pub password: String,
}

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize, Debug)]
#[ts(export)]
pub struct Book {
    pub name: String,
    pub year: i32,
    pub writer: String,
}
