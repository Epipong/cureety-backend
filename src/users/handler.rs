use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::{
    dsl::insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper
};
use uuid::Uuid;

use crate::{
    schema::users::{self, dsl::*},
    users::model::{Pool, Roles, User}, utils::hash_password,
};

#[get("/users")]
pub async fn users_list(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let items = users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
        .expect("select");

    HttpResponse::Ok().json(items)
}

#[post("/users")]
pub async fn create_user(body: web::Json<User>, pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let items = users::table
        .select(User::as_select())
        .load::<User>(&mut conn)
        .expect("select");

    let hashed = hash_password(body.hash.as_str()).unwrap();
    let datetime = Utc::now().naive_utc();

    let created_user = User {
        id: Uuid::new_v4(),
        username: body.username.clone(),
        email: body.email.clone(),
        hash: hashed,
        role: Roles::Patient,
        created_at: datetime,
        updated_at: datetime,
    };


    HttpResponse::Ok().json(created_user)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(users_list)
        .service(create_user);
    conf.service(scope);
}
