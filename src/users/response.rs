use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
use super::models::Roles;

#[derive(Serialize, Debug)]
pub struct UserResponse {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub role: Roles,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}
