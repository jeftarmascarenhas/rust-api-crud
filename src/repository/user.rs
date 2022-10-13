
use crate::models::user::User;

pub fn get_users() -> Vec<User> {
  let users = vec![User {
    name: String::from("John"),
    email: String::from("John"),
  }];
  users
}