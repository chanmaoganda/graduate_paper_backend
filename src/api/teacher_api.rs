use axum::{
    routing::{get, post},
    Router,
};

use super::{LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT, UNREGISTER_ENDPOINT};

pub fn get_teacher_router() -> Router {
    let query_api = get(get_services::get_teacher_by_id);
    let list_api = get(get_services::list_all_teachers);
    let register_api = post(post_services::register_teacher);
    let unregister_api = post(post_services::unregister_teacher);

    Router::new()
        .route(QUERY_ID_ENDPOINT, query_api)
        .route(LIST_ENDPOINT, list_api)
        .route(REGISTER_ENDPOINT, register_api)
        .route(UNREGISTER_ENDPOINT, unregister_api)
}

mod get_services {
    use std::sync::Arc;

    use axum::{
        extract::Query,
        response::{IntoResponse, Response},
        Extension,
    };
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;

    use crate::model::{QueryById, Teacher};

    use crate::api::TEACHER_TABLE;

    pub async fn get_teacher_by_id(
        teacher_id: Query<QueryById>,
        pool: Extension<Arc<Pool>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("get teacher by teacher_id");

        let query = format!("SELECT name FROM {TEACHER_TABLE} WHERE teacher_id = $1;");
        let stmt = client.prepare(&query).await.unwrap();
        match client.query_one(&stmt, &[&teacher_id.inner]).await {
            Ok(row) => {
                let name: String = row.get(0);
                Response::new(name)
            }
            Err(_) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Teacher not found".into())
                .unwrap(),
        }
    }

    pub async fn list_all_teachers(pool: Extension<Arc<Pool>>) -> impl IntoResponse {
        let client = pool.get().await.unwrap();

        log::debug!("get all teachers");

        let query = format!("SELECT teacher_id, name, email FROM {TEACHER_TABLE};");
        let stmt = client.prepare(&query).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let teachers = rows
            .into_iter()
            .map(Teacher::from_row)
            .collect::<Vec<Teacher>>();
        axum::Json(teachers)
    }
}

mod post_services {
    use std::sync::Arc;

    use axum::response::Response;
    use axum::{Extension, Json};
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;

    use crate::manager::RegexManager;
    use crate::model::Teacher;

    use crate::api::TEACHER_TABLE;

    pub async fn register_teacher(
        pool: Extension<Arc<Pool>>,
        regex: Extension<Arc<RegexManager>>,
        Json(teachers): Json<Vec<Teacher>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        let filter_result = teachers.iter().all(|teacher| teacher.check_valid(&regex));
        if !filter_result {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid id or email!".into())
                .unwrap();
        }

        for teacher in teachers.iter() {
            let Teacher {
                teacher_id,
                name,
                password,
                ..
            } = teacher;

            let query = if let Some(email) = &teacher.email {
                format!("INSERT INTO {TEACHER_TABLE} (teacher_id, name, password, email) VALUES ($1, $2, $3, '{email}');")
            } else {
                format!("INSERT INTO {TEACHER_TABLE} (teacher_id, name, password) VALUES ($1, $2, $3);")
            };

            let stmt = client.prepare(&query).await.unwrap();
            if client
                .execute(&stmt, &[&teacher_id, &name, &password])
                .await
                .is_err()
            {
                log::error!("Error registering teacher: {}, {}", teacher_id, name);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering teacher".into())
                    .unwrap();
            }
        }

        Response::new("Teachers registered".into())
    }

    // TODO: unregistering teacher requires privileges
    pub async fn unregister_teacher(
        pool: Extension<Arc<Pool>>,
        Json(teachers): Json<Vec<Teacher>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("unregister teacher");

        for teacher in teachers {
            let Teacher {
                teacher_id,
                name,
                password,
                ..
            } = teacher;

            let query = format!("DELETE FROM {TEACHER_TABLE} WHERE teacher_id = $1 AND name = $2 AND password = $3;");

            let stmt = client.prepare(&query).await.unwrap();
            if client
                .execute(&stmt, &[&teacher_id, &name, &password])
                .await
                .is_err()
            {
                log::error!("Error registering teacher: {}, {}", teacher_id, name);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering teacher".into())
                    .unwrap();
            }
        }
        Response::new("Teachers unregistered".into())
    }
}
