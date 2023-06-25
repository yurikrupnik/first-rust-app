use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize)]
#[ts(export)]
pub struct User {
    // pub _id: String,
    // pub _id: Option<String>,
    pub name: String,
    pub role: String,
    pub age: i32,
    pub email: String,
    // pub has_car: bool,
    // pub password: String,
}

#[derive(Clone, TS, PartialEq, Eq, Deserialize, Serialize)]
#[ts(export)]
pub struct Book {
    // pub _id: String,
    // pub _id: Option<String>,
    pub name: String,
    pub year: i32,
    pub writer: String,
    // pub has_car: bool,
    // pub password: String,
}

// impl User {
//     fn full_name(&self) -> String {
//         return self.name;
//     }
//     pub fn new(&self) -> &self {
//         return self { name: "" };
//     }
//     pub fn age(&self) -> i32 {
//         self.age + 2
//     }
// }
// #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
// pub struct List {
//     list: Vec<User>,
// }
//
// pub struct CreateUserDto {
//     pub email: String,
//     pub password: String,
// }
