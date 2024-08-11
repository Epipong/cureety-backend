use actix_web::{body, get, post, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    schema::users,
    users::model::{Pool, User},
};

use super::model::AppState;

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
pub async fn create_user(
    mut body: web::Json<User>,
    data: web::Data<AppState>
) -> impl Responder {
    let mut vec = data.user_db.lock().unwrap();

    HttpResponse::Ok().json(0)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(users_list);
    conf.service(scope);
}
