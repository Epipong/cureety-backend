use serde::Serialize;
use super::model::User;

#[derive(Serialize, Debug)]
pub struct UserListResponse {
  pub status: String,
  pub results: usize,
  pub users: Vec<User>,
}