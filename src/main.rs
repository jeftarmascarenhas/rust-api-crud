#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod api;
mod repository;
pub mod models;

use api::user::*;
use rocket::{Rocket, Build};


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/api/v1", routes![index, hello])
}

#[cfg(test)]
mod tests {
  use super::rocket;
  use rocket::local::blocking::Client;
  use rocket::http::Status;

  #[test] 
  fn hello() {
    let name = "Jeff";
    let age = 36;
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!("/api/v1", super::hello(name, age))).dispatch();

    let result = format!("Hello, {} years old named {}!", age, name);

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), result);
  }
}
