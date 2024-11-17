use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Extension, Router};
use deadpool_postgres::Pool;

use crate::model::FullPaperData;

use super::{LIST_ENDPOINT, PAPER_TABLE, STUDENT_TABLE, TEACHER_TABLE};

pub fn get_full_paper_router() -> Router {
    let list_api = get(list_all_full_paper);

    Router::new()
        .route(LIST_ENDPOINT, list_api)
}

async fn list_all_full_paper(pool: Extension<Arc<Pool>>) -> impl IntoResponse {
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
    axum::Json(full_paper_datas)
}
