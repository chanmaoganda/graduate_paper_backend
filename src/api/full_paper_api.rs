use actix_web::{web, Responder};
use deadpool_postgres::Pool;

use crate::model::FullPaperData;

use super::{LIST_ENDPOINT, PAPER_TABLE, STUDENT_TABLE, TEACHER_TABLE};

pub fn get_full_paper_apis() -> actix_web::Scope {
    let list_api = web::resource(LIST_ENDPOINT).route(web::get().to(list_all_paper));

    web::scope("/full_paper").service(list_api)
}

async fn list_all_paper(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();
    log::debug!("list all papers");

    let sql = format!("SELECT p.base_id, s.name, s.student_id, s.email, p.title, t.name, t.teacher_id, t.email FROM {PAPER_TABLE} AS p \
        JOIN {STUDENT_TABLE} AS s ON p.student_id = s.student_id JOIN {TEACHER_TABLE} AS t ON p.teacher_id = t.teacher_id;");
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let full_paper_datas = rows
        .into_iter()
        .map(FullPaperData::from_row)
        .collect::<Vec<FullPaperData>>();
    web::Json(full_paper_datas)
}
