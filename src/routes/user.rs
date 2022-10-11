use rocket::serde::json::Json;
use rocket::http::Status;

use crate::entities::user::User;
use crate::services::user::get_users;



#[get("/",)]
pub fn index() -> (Status, Json<Vec<User>>)  {
    (Status::Ok, Json(get_users()))
}

#[get("/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} Year old named {}!", age, name)
}
