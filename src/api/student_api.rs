use axum::{
    routing::{get, post},
    Router,
};

use super::{LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT, UNREGISTER_ENDPOINT};

pub fn get_student_router() -> Router {
    let query_api = get(get_services::get_student_by_id);
    let list_api = get(get_services::list_all_students);
    let register_api = post(post_services::register_student);
    let unregister_api = post(post_services::unregister_student);

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
        Extension, Json,
    };
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;

    use crate::model::{BasicUser, QueryById};

    use crate::api::STUDENT_TABLE;

    pub async fn get_student_by_id(
        Query(id): Query<QueryById>,
        pool: Extension<Arc<Pool>>,
    ) -> impl IntoResponse {
        let client = pool.get().await.unwrap();

        log::debug!("get student by id");

        let query = format!("SELECT id, name, email FROM {STUDENT_TABLE} WHERE id = $1;");
        let stmt = client.prepare(&query).await.unwrap();
        match client.query_one(&stmt, &[&id.inner]).await {
            Ok(row) => {
                Json(BasicUser::from_row(row)).into_response()
            }
            Err(_) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Student not found".into())
                .unwrap(),
        }
    }

    pub async fn list_all_students(pool: Extension<Arc<Pool>>) -> impl IntoResponse {
        let client = pool.get().await.unwrap();

        log::debug!("get all students");

        let query = format!("SELECT id, name, email FROM {STUDENT_TABLE}");
        let stmt = client.prepare(&query).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let students = rows
            .into_iter()
            .map(BasicUser::from_row)
            .collect::<Vec<BasicUser>>();
        axum::Json(students)
    }
}

mod post_services {
    use std::sync::Arc;

    use axum::response::Response;
    use axum::{Extension, Json};
    use deadpool_postgres::Pool;
    use reqwest::StatusCode;

    use crate::manager::RegexManager;
    use crate::model::Student;

    use crate::api::STUDENT_TABLE;

    // register json requires efficient inserting like VALUES ('{}', '{}') (...) (...)
    pub async fn register_student(
        pool: Extension<Arc<Pool>>,
        regex: Extension<Arc<RegexManager>>,
        Json(students): Json<Vec<Student>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        let filter_result = students.iter().all(|student| student.check_valid(&regex));
        if !filter_result {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid id or email!".into())
                .unwrap();
        }

        for student in students.iter() {
            let Student {
                id,
                name,
                password,
                email,
            } = student;

            let query = if let Some(email) = &email {
                format!("INSERT INTO {STUDENT_TABLE} (id, name, password, email) VALUES ($1, $2, $3, '{email}');")
            } else {
                format!("INSERT INTO {STUDENT_TABLE} (id, name, password) VALUES ($1, $2, $3);")
            };

            let stmt = client.prepare(&query).await.unwrap();
            if client
                .execute(&stmt, &[&id, &name, &password])
                .await
                .is_err()
            {
                log::error!("Error registering student: {}, {}", name, id);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering student".into())
                    .unwrap();
            }
        }

        Response::new("Students registered".into())
    }

    pub async fn unregister_student(
        pool: Extension<Arc<Pool>>,
        Json(students): Json<Vec<Student>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("unregister student");

        for student in students {
            let Student {
                id,
                name,
                password,
                ..
            } = student;

            let query = format!("DELETE FROM {STUDENT_TABLE} WHERE id = $1 AND name = $2 AND password = $3;");

            let stmt = client.prepare(&query).await.unwrap();
            if client
                .execute(&stmt, &[&id, &name, &password])
                .await
                .is_err()
            {
                log::error!("Error registering student: {}, {}", name, id);

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering student".into())
                    .unwrap();
            }
        }
        Response::new("Students unregistered".into())
    }
}
