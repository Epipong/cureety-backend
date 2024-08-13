use serde::Serialize;
use super::models::User;

#[derive(Serialize, Debug)]
pub struct UserListResponse {
  pub status: String,
  pub results: usize,
  pub users: Vec<User>,
}

#[derive(Serialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
