use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
  pub name: String,
}