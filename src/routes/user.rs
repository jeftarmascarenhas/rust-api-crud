use rocket::{serde::json::Json};
use rocket::http::{Status, ContentType};

use crate::{entities::user::User, services::user::get_users};



#[get("/")]
pub fn index() -> (Status, (ContentType, Json<Vec<User>>))  {
    (Status::Ok, (ContentType::JSON, Json(get_users())))
}

#[get("/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} Year old named {}!", age, name)
}
