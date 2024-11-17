use axum::{
    routing::{get, post},
    Router,
};

use super::{
    JSON_REGISTER_ENDPOINT, LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT,
    UNREGISTER_ENDPOINT,
};

pub fn get_student_router() -> Router {
    let query_api = get(get_services::get_student_by_id);
    let list_api = get(get_services::list_all_students);
    let register_api = post(post_services::register_student);
    let json_register_api = post(post_services::register_student_json);
    let unregister_api = post(post_services::unregister_student);

    Router::new()
        .route(QUERY_ID_ENDPOINT, query_api)
        .route(LIST_ENDPOINT, list_api)
        .route(REGISTER_ENDPOINT, register_api)
        .route(JSON_REGISTER_ENDPOINT, json_register_api)
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

    use crate::model::{QueryById, Student};

    use crate::api::STUDENT_TABLE;

    pub async fn get_student_by_id(
        student_id: Query<QueryById>,
        pool: Extension<Arc<Pool>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("get student by student_id");

        let sql = format!("SELECT name FROM {STUDENT_TABLE} WHERE student_id = $1;");
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.query_one(&stmt, &[&student_id.inner]).await {
            Ok(row) => {
                let name: String = row.get(0);
                Response::new(name)
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

        let sql = format!("SELECT student_id, name, email FROM {STUDENT_TABLE}");
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let students = rows
            .into_iter()
            .map(Student::from_row)
            .collect::<Vec<Student>>();
        axum::Json(students)
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
    use crate::model::Student;

    use crate::api::STUDENT_TABLE;

    pub async fn register_student(
        student: Query<Student>,
        pool: Extension<Arc<Pool>>,
        regex: Extension<Arc<RegexManager>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        if !student.check_valid(&regex) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid id or email!".into())
                .unwrap();
        }

        let Student {
            student_id,
            name,
            email,
        } = student.0;

        let sql = if let Some(email) = email {
            format!(
                "INSERT INTO {STUDENT_TABLE} (student_id, name, email) VALUES ($1, $2, '{email}');"
            )
        } else {
            format!("INSERT INTO {STUDENT_TABLE} (student_id, name) VALUES ($1, $2);")
        };

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[&student_id, &name]).await {
            Ok(_) => Response::new("Student registered".into()),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Error registering student".into())
                .unwrap(),
        }
    }

    pub async fn unregister_student(
        student: Query<Student>,
        pool: Extension<Arc<Pool>>,
    ) -> Response<String> {
        let client = pool.get().await.unwrap();

        log::debug!("unregister student");

        let Student {
            student_id, name, ..
        } = student.0;

        let sql = format!("DELETE FROM {STUDENT_TABLE} WHERE student_id = $1 AND name = $2;");

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[&student_id, &name]).await {
            Ok(_) => Response::new("Student unregistered".into()),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error unregister student".into())
                .unwrap(),
        }
    }

    // register json requires efficient inserting like VALUES ('{}', '{}') (...) (...)
    pub async fn register_student_json(
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
            let sql = if let Some(email) = &student.email {
                format!("INSERT INTO {STUDENT_TABLE} (student_id, name, email) VALUES ($1, $2, '{email}');")
            } else {
                format!("INSERT INTO {STUDENT_TABLE} (student_id, name) VALUES ($1, $2);")
            };

            let stmt = client.prepare(sql.as_str()).await.unwrap();
            if client
                .execute(&stmt, &[&student.student_id, &student.name])
                .await
                .is_err()
            {
                log::error!(
                    "Error registering student: {}, {}",
                    student.name,
                    student.student_id
                );

                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error registering student".into())
                    .unwrap();
            }
        }

        Response::new("Students registered".into())
    }
}
