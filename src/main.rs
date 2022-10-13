#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod api;
mod repository;
pub mod models;

use api::user::*;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![index, hello])
}
