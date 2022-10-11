#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod services;
pub mod entities;

use routes::user::*;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1", routes![index, hello])
}
