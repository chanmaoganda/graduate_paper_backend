use std::sync::Arc;

use axum::{extract::Query, response::Response, routing::get, Extension, Router};
use deadpool_postgres::Pool;
use reqwest::StatusCode;

use crate::model::QueryById;

use super::{STUDENT_TABLE, TEACHER_TABLE};

pub fn get_login_router() -> Router {
    let student_login_api = get(student_login);
    let teacher_login_api = get(teacher_login);

    Router::new()
        .route("/student", student_login_api)
        .route("/teacher", teacher_login_api)
}

async fn student_login(user_id: Query<QueryById>, pool: Extension<Arc<Pool>>) -> Response<String> {
    base_login(user_id, STUDENT_TABLE, pool).await
}

async fn teacher_login(user_id: Query<QueryById>, pool: Extension<Arc<Pool>>) -> Response<String> {
    base_login(user_id, TEACHER_TABLE, pool).await
}

async fn base_login(
    user_id: Query<QueryById>,
    table_name: &str,
    pool: Extension<Arc<Pool>>,
) -> Response<String> {
    let client = pool.get().await.unwrap();

    let sql = format!(
        "SELECT COUNT(1) FROM {table_name} WHERE {table_name}_id = '{}';",
        user_id.inner
    );
    let row = client.query_one(&sql, &[]).await.unwrap();

    let count: i64 = row.get(0);
    if count != 1 {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid username or password".into())
            .unwrap()
    } else {
        let body = format!("{table_name} {} login successful", user_id.inner);
        Response::new(body)
    }
}
