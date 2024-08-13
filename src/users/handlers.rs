use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, ResponseError};
use diesel::{
    dsl::{delete, insert_into},
    update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    schema::users::{self, dsl::*},
    users::models::{Pool, User, UserCreate},
};

use super::models::UserEdit;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();

    match users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
    {
        Ok(users_list) => HttpResponse::Ok().json(&users_list),
        Err(error) => ServiceError::BadRequest(error.to_string()).error_response(),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = db.get().unwrap();

    match users
        .find(user_id.into_inner())
        .get_result::<User>(&mut conn)
    {
        Ok(user) => HttpResponse::Ok().json(&user),
        Err(error) => ServiceError::BadRequest(error.to_string()).error_response(),
    }
}

#[post("/users")]
pub async fn add_user(body: web::Json<UserCreate>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let new_user = User::from_create(&body);

    match insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(created_user) => HttpResponse::Ok().json(&created_user),
        Err(error) => ServiceError::BadRequest(error.to_string()).error_response(),
    }
}

#[patch("/users/{id}")]
pub async fn update_user(
    path: web::Path<Uuid>,
    body: web::Json<UserEdit>,
    db: web::Data<Pool>,
) -> impl Responder {
    let mut conn = db.get().unwrap();
    let user_id = path.into_inner();
    let updated_user = User::from_edit(&body);

    match update(users)
        .filter(id.eq(user_id))
        .set(&updated_user)
        .get_result::<User>(&mut conn)
    {
        Ok(edited_user) => HttpResponse::Ok().json(&edited_user),
        Err(error) => ServiceError::BadRequest(error.to_string()).error_response(),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(path: web::Path<Uuid>, db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let user_id = path.into_inner();

    match delete(users)
        .filter(id.eq(user_id))
        .get_result::<User>(&mut conn)
    {
        Ok(deleted_user) => HttpResponse::Ok().json(&deleted_user),
        Err(error) => ServiceError::BadRequest(error.to_string()).error_response(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_users)
        .service(get_user_by_id)
        .service(add_user)
        .service(update_user)
        .service(delete_user);
    conf.service(scope);
}
