use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::{
    dsl::{delete, insert_into}, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::{
    schema::users::{self, dsl::*},
    users::model::{Pool, Roles, User, UserCreate},
    utils::hash_password,
};

use super::model::UserEdit;

#[get("/users")]
pub async fn users_list(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    match users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
    {
        Ok(users_list) => HttpResponse::Ok().json(&users_list),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

#[post("/users")]
pub async fn create_user(body: web::Json<UserCreate>, pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    let datetime = Utc::now().naive_utc();

    // TODO: only admin can assign roles
    let user_role = if let Some(user_role) = &body.role {
        user_role
    } else {
        &Roles::Patient
    };

    let new_user = User {
        id: Uuid::new_v4(),
        username: body.username.clone(),
        email: body.email.clone(),
        hash: hash_password(&body.hash).unwrap(),
        role: user_role.clone(),
        created_at: datetime,
        updated_at: datetime,
    };

    match insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(created_user) => HttpResponse::Ok().json(&created_user),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

#[patch("/users/{user_id}")]
pub async fn edit_user(
    path: web::Path<Uuid>,
    body: web::Json<UserEdit>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let user_id = path.into_inner();

    let user_hash = if let Some(hashed) = &body.hash {
        Some(hash_password(hashed.as_str()).unwrap())
    } else {
        None
    };

    // TODO: only admin can assign roles
    let user_role = if let Some(user_role) = &body.role {
        Some(user_role.clone())
    } else {
        None
    };

    let updated_user = UserEdit {
        username: body.username.clone(),
        email: body.email.clone(),
        hash: user_hash,
        role: user_role,
        updated_at: Some(Utc::now().naive_utc()),
    };

    match update(users)
        .filter(id.eq(user_id))
        .set(&updated_user)
        .get_result::<User>(&mut conn)
    {
        Ok(edited_user) => HttpResponse::Ok().json(&edited_user),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(path: web::Path<Uuid>, pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let user_id = path.into_inner();

    match delete(users).filter(id.eq(user_id)).get_result::<User>(&mut conn) {
        Ok(deleted_user) => HttpResponse::Ok().json(&deleted_user),
        Err(error) => HttpResponse::BadRequest().json(error.to_string()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(users_list)
        .service(create_user)
        .service(edit_user)
        .service(delete_user);
    conf.service(scope);
}
