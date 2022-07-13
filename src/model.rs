use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String,
}

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
// pub struct List {
//     list: Vec<User>,
// }
//
// pub struct CreateUserDto {
//     pub email: String,
//     pub password: String,
// }
