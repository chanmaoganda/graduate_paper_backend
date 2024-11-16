use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::model::User;

use super::{STUDENT_TABLE, TEACHER_TABLE};

pub fn get_login_api() -> actix_web::Scope {
    let student_login = web::resource("/student").route(web::get().to(student_login));
    let teacher_login = web::resource("/teacher").route(web::get().to(teacher_login));

    web::scope("/login")
        .service(student_login)
        .service(teacher_login)
}

async fn student_login(user: web::Query<User>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    let username = user.into_inner().id;

    let sql = format!("SELECT COUNT(1) FROM {STUDENT_TABLE} WHERE student_id = '{username}';");
    let row = client.query_one(&sql, &[]).await.unwrap();

    let count: i64 = row.get(0);
    log::debug!("count: {}", count);
    
    if count != 1 {
        HttpResponse::Unauthorized().body("Invalid username or password")
    } else {
        HttpResponse::Ok().body("Login successful")
    }
}

async fn teacher_login(user: web::Query<User>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    let username = user.into_inner().id;

    let sql = format!("SELECT COUNT(1) FROM {TEACHER_TABLE} WHERE teacher_id = '{username}';");
    let row = client.query_one(&sql, &[]).await.unwrap();

    let count: i64 = row.get(0);
    if count != 1 {
        HttpResponse::Unauthorized().body("Invalid username or password")
    } else {
        HttpResponse::Ok().body("Login successful")
    }
}