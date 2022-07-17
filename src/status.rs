use actix_web::{get, Responder};

// async fn get_json() -> String {
//     return "hello".into();
// }

#[get("/status")]
pub async fn status() -> impl Responder {
    "{\"status\": \"up\"}"
}
//
// fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
//     let mut largest = number_list[0];
//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     largest
// }
//
// fn todo_and_remove<T>(a: T) -> T {
//     a
// }
