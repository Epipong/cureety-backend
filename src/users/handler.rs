use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::{dsl::insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    schema::users::{self, dsl::*},
    users::model::{Pool, Roles, User, UserCreate},
    utils::hash_password,
};

#[get("/users")]
pub async fn users_list(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    match users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

#[post("/users")]
pub async fn create_user(body: web::Json<UserCreate>, pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    let hashed = hash_password(body.hash.as_str()).unwrap();
    let datetime = Utc::now().naive_utc();

    if let Some(user_role) = &body.role {
        log::info!("role is {}", user_role.as_str());
        todo!();
    }

    let new_user = User {
        id: Uuid::new_v4(),
        username: body.username.clone(),
        email: body.email.clone(),
        hash: hashed,
        role: Roles::Patient,
        created_at: datetime,
        updated_at: datetime,
    };

    match insert_into(users).values(&new_user).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json(new_user),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(users_list).service(create_user);
    conf.service(scope);
}
