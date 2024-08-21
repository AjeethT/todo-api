use todo::TodoManager;

// use std::time::Duration;
mod todo;
// use rocket::tokio::time::sleep;

// #[macro_use]
// extern crate rocket;

// #[get("/world")]
// fn index() -> &'static str {
//     "Hello, world"
// }

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited {}", seconds)
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/", routes![delay])
// }

fn main() {
    let todo = todo::Todo {
        item_id: 1,
        item_desc: "Learn Rust".to_string(),
        item_tags: "programming,rust".to_string(),
        is_active: true,
        remainder_date: "2022-12-31".to_string(),
        created_date: "2022-12-20".to_string(),
        modified_date: "2022-12-20".to_string(),
    };

    todo.create_item()
}