
use crate::entities::user::User;

pub fn get_users() -> Vec<User> {
  let users = vec![User {
    name: String::from("John"),
  }];
  users
}