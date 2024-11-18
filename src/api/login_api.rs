use std::sync::Arc;

use axum::{response::Response, routing::get, Extension, Json, Router};
use deadpool_postgres::Pool;
use reqwest::StatusCode;

use crate::model::LoginUser;

use super::{STUDENT_TABLE, TEACHER_TABLE};

pub fn get_login_router() -> Router {
    let student_login_api = get(student_login);
    let teacher_login_api = get(teacher_login);

    Router::new()
        .route("/student", student_login_api)
        .route("/teacher", teacher_login_api)
}

async fn student_login(pool: Extension<Arc<Pool>>, user: Json<LoginUser>) -> Response<String> {
    base_login(STUDENT_TABLE, pool, user).await
}

async fn teacher_login(pool: Extension<Arc<Pool>>, user: Json<LoginUser>) -> Response<String> {
    base_login(TEACHER_TABLE, pool, user).await
}

async fn base_login(
    table_name: &str,
    pool: Extension<Arc<Pool>>,
    user: Json<LoginUser>,
) -> Response<String> {
    let client = pool.get().await.unwrap();

    let query =
        format!("SELECT COUNT(1) FROM {table_name} WHERE id = $1 AND password = $2;");
    let row = client
        .query_one(&query, &[&user.id, &user.password])
        .await
        .unwrap();

    let count: i64 = row.get(0);
    if count != 1 {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid username or password".into())
            .unwrap()
    } else {
        let body = format!("{table_name} {} login successful", user.id);
        Response::new(body)
    }
}
