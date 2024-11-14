use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::model::Student;

const STUDENT_TABLE: &str = env!("STUDENT_TABLE");

pub fn get_student_apis() -> actix_web::Scope {
    let list_api = web::resource("/list").route(web::get().to(list_all_students));
    let query_id_api = web::resource("/query/{id}").route(web::get().to(get_student_by_id));

    web::scope("/student")
        .service(list_api)
        .service(query_id_api)
}

async fn get_student_by_id(student_id: web::Path<u32>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get student by student_id");
    let student_id = student_id.into_inner();
    let sql = format!("SELECT name FROM {STUDENT_TABLE} WHERE student_id = '{}';", student_id);
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    match client.query_one(&stmt, &[]).await {
        Ok(row) => {
            let name: String = row.get(0);
            HttpResponse::Ok().json(name)
        }
        Err(_) => HttpResponse::NotFound().body("Student not found"),
    }
}

async fn list_all_students(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get all students");
    let sql = format!("SELECT base_id, name, student_id, email FROM {STUDENT_TABLE}");
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let students = rows
        .into_iter()
        .map(Student::from_row)
        .collect::<Vec<Student>>();
    web::Json(students)
}
