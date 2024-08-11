use actix_web::{get, web, HttpResponse, Responder};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{
    schema::users,
    users::model::{Pool, User},
};

#[get("/users")]
pub async fn users_list(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let items = users::table.select(User::as_select()).load::<User>(&mut conn).unwrap();

    HttpResponse::Ok().json(items)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(users_list);
    conf.service(scope);
}
