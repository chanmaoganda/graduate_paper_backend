use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Router};
use deadpool_postgres::Pool;

use crate::model::{Paper, QueryById};

use super::{LIST_ENDPOINT, PAPER_TABLE, QUERY_ID_ENDPOINT};

pub fn get_paper_router() -> Router {
    let query_id_api = get(get_paper_by_id);
    let list_api = get(list_all_paper);

    Router::new()
        .route(QUERY_ID_ENDPOINT, query_id_api)
        .route(LIST_ENDPOINT, list_api)
}

async fn get_paper_by_id(
    student_id: Query<QueryById>,
    pool: Extension<Arc<Pool>>,
) -> impl IntoResponse {
    let client = pool.get().await.unwrap();

    log::debug!("get paper by student_id");

    let sql = format!(
        "SELECT title FROM {PAPER_TABLE} WHERE student_id = '{}';",
        student_id.inner
    );
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    match client.query_one(&stmt, &[]).await {
        Ok(row) => {
            let name: String = row.get(0);
            axum::Json(name)
        }
        Err(_) => axum::Json("not found".to_string()),
    }
}

async fn list_all_paper(pool: Extension<Arc<Pool>>) -> impl IntoResponse {
    let client = pool.get().await.unwrap();

    log::debug!("get all paper");
    let sql = format!("SELECT base_id, student_id, teacher_id, title FROM {PAPER_TABLE};");
    let stmt = client.prepare(sql.as_str()).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let paper = rows
        .into_iter()
        .map(Paper::from_row)
        .collect::<Vec<Paper>>();
    axum::Json(paper)
}
