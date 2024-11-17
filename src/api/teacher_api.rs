use axum::{
    routing::{get, post},
    Router,
};

use super::{JSON_REGISTER_ENDPOINT, LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT, UNREGISTER_ENDPOINT};

pub fn get_teacher_router() -> Router {
    let query_api = get(get_services::get_teacher_by_id);
    let list_api = get(get_services::list_all_teachers);
    let register_api = post(post_services::register_teacher);
    let json_register_api = post(post_services::register_teacher_json);
    let unregister_api = post(post_services::unregister_teacher);

    Router::new()
        .route(QUERY_ID_ENDPOINT, query_api)
        .route(LIST_ENDPOINT, list_api)
        .route(REGISTER_ENDPOINT, register_api)
        .route(JSON_REGISTER_ENDPOINT, json_register_api)
        .route(UNREGISTER_ENDPOINT, unregister_api)
}

mod get_services {
    use std::sync::Arc;

    use axum::extract::Query;
    use axum::response::{IntoResponse, Response};
    use axum::Extension;
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

        let sql = format!(
            "SELECT name FROM {TEACHER_TABLE} WHERE teacher_id = '{}';",
            teacher_id.inner
        );
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.query_one(&stmt, &[]).await {
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
        let sql = format!("SELECT name, teacher_id, email FROM {TEACHER_TABLE};");
        let stmt = client.prepare(sql.as_str()).await.unwrap();
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

    use axum::extract::Query;
    use axum::response::Response;
    use axum::{Extension, Json};
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;

    use crate::manager::RegexManager;
    use crate::model::Teacher;

    use crate::api::TEACHER_TABLE;

    // TODO: registering teacher requires privileges
    pub async fn register_teacher(
        teacher: Query<Teacher>,
        pool: Extension<Arc<Pool>>,
        regex: Extension<Arc<RegexManager>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("create teacher");

        if !teacher.check_valid(&regex) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid email or password".into())
                .unwrap();
        }

        let Teacher {
            teacher_id,
            name,
            email,
        } = teacher.0;

        let sql = if let Some(email) = email {
            format!(
                "INSERT INTO {TEACHER_TABLE} (name, teacher_id, email) VALUES ('{}', '{}', '{}');",
                name, teacher_id, email
            )
        } else {
            format!(
                "INSERT INTO {TEACHER_TABLE} (name, teacher_id) VALUES ('{}', '{}');",
                name, teacher_id
            )
        };

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[]).await {
            Ok(_) => Response::new("Student created".into()),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Error creating student".into())
                .unwrap(),
        }
    }

    pub async fn unregister_teacher(
        teacher: Query<Teacher>,
        pool: Extension<Arc<Pool>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("unregister teacher");

        let Teacher {
            teacher_id, name, ..
        } = teacher.0;

        let sql = format!(
            "DELETE FROM {TEACHER_TABLE} WHERE teacher_id = '{}' AND name = '{}';",
            teacher_id, name
        );

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[]).await {
            Ok(_) => Response::new("Teacher unregistered".into()),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal error unregister teacher".into())
                .unwrap(),
        }
    }

    pub async fn register_teacher_json(
        pool: Extension<Arc<Pool>>,
        regex: Extension<Arc<RegexManager>>,
        Json(teachers): Json<Vec<Teacher>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        let filter_result = teachers.iter()
            .all(|teacher| teacher.check_valid(&regex));
        if !filter_result {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid id or email!".into())
                .unwrap();
        }

        for teacher in teachers.iter() {
            let sql = if let Some(email) = &teacher.email {
                format!(
                    "INSERT INTO {TEACHER_TABLE} (name, teacher_id, email) VALUES ('{}', '{}', '{}');",
                    teacher.name, teacher.teacher_id, email
                )
            } else {
                format!(
                    "INSERT INTO {TEACHER_TABLE} (name, teacher_id) VALUES ('{}', '{}');",
                    teacher.name, teacher.teacher_id
                )
            };

            let stmt = client.prepare(sql.as_str()).await.unwrap();
            if client.execute(&stmt, &[]).await.is_err() {
                log::error!("Error registering student: {}, {}", teacher.name, teacher.teacher_id);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering teacher".into())
                    .unwrap();
            }
        }

        Response::new("Teachers registered".into())
    }
}
