use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::model::Paper;

const PAPER_TABLE: &str = env!("PAPER_TABLE");

pub fn get_paper_apis() -> actix_web::Scope {
    let query_id_api = web::resource("/query/{id}").route(web::get().to(get_paper_by_id));
    let list_api = web::resource("/list").route(web::get().to(list_all_paper));

    web::scope("/paper")
       .service(query_id_api)
       .service(list_api)
}

async fn get_paper_by_id(student_id: web::Path<u32>, pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get paper by student_id");
    let student_id = student_id.into_inner();
    let sql = format!("SELECT title FROM {PAPER_TABLE} WHERE student_id = '{}';", student_id);
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    match client.query_one(&stmt, &[]).await {
        Ok(row) => {
            let name: String = row.get(0);
            HttpResponse::Ok().json(name)
        }
        Err(_) => HttpResponse::NotFound().body("Paper not found"),
    }
}

async fn list_all_paper(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();

    log::debug!("get all paper");
    let sql = format!("SELECT base_id, student_id, teacher_id, title FROM {PAPER_TABLE};");
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let paper = rows
        .into_iter()
        .map(Paper::from_row)
        .collect::<Vec<Paper>>();
    web::Json(paper)
}
