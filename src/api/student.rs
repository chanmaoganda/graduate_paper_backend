use actix_web::{web::{self}, HttpResponse, Responder};
use deadpool_postgres::Pool;

pub async fn get_student_by_id(id: web::Path<u32>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("getting into response");
    let id = id.into_inner();
    let sql = format!("SELECT name FROM student WHERE id = {};", id);
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    match client.query_one(&stmt, &[]).await {
        Ok(row) => {
            let name: String = row.get(0);
            HttpResponse::Ok().json(name)
        },
        Err(_) => {
            HttpResponse::NotFound().body("Student not found")
        }
    }
}