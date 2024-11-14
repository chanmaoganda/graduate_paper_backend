use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::model::Teacher;

use super::{LIST_ENDPOINT, QUERY_ID_ENDPOINT, TEACHER_TABLE};

pub fn get_teacher_apis() -> actix_web::Scope {
    let query_id_api = web::resource(QUERY_ID_ENDPOINT).route(web::get().to(get_teacher_by_id));
    let list_api = web::resource(LIST_ENDPOINT).route(web::get().to(list_all_teachers));

    web::scope("/teacher")
        .service(list_api)
        .service(query_id_api)
}

async fn get_teacher_by_id(teacher_id: web::Path<u32>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get teacher by teacher_id");
    let teacher_id = teacher_id.into_inner();
    let sql = format!(
        "SELECT name FROM {TEACHER_TABLE} WHERE teacher_id = '{}';",
        teacher_id
    );
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    match client.query_one(&stmt, &[]).await {
        Ok(row) => {
            let name: String = row.get(0);
            HttpResponse::Ok().json(name)
        }
        Err(_) => HttpResponse::NotFound().body("Teacher not found"),
    }
}

async fn list_all_teachers(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get all teachers");
    let sql = format!("SELECT base_id, name, teacher_id, email FROM {TEACHER_TABLE};");
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let teachers = rows
        .into_iter()
        .map(Teacher::from_row)
        .collect::<Vec<Teacher>>();
    web::Json(teachers)
}
