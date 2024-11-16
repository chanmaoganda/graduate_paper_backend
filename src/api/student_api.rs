use actix_web::web;

use super::{QUERY_ID_ENDPOINT, LIST_ENDPOINT, STUDENT_TABLE, REGISTER_ENDPOINT};

pub fn get_student_apis() -> actix_web::Scope {
    let query_id_api = web::resource(QUERY_ID_ENDPOINT)
        .route(web::get().to(get_services::get_student_by_id));

    let list_api = web::resource(LIST_ENDPOINT)
        .route(web::get().to(get_services::list_all_students));
    
    let register_student_api = web::resource(REGISTER_ENDPOINT)
        .route(web::post().to(post_services::register_student));

    web::scope("/student")
        .service(query_id_api)
        .service(list_api)
        .service(register_student_api)
}

mod get_services {
    use actix_web::{web, Responder, HttpResponse};
    use deadpool_postgres::Pool;
    use serde::{Deserialize, Serialize};
    use crate::model::Student;

    use super::super::STUDENT_TABLE;

    #[derive(Serialize, Deserialize)]
    pub struct StudentId {
        #[serde(rename = "id")]
        pub inner: String,
    }

    pub async fn get_student_by_id(student_id: web::Query<StudentId>, pool: web::Data<Pool>) -> impl Responder {
        let client = pool.get().await.unwrap();
    
        log::debug!("get student by student_id");
        let student_id = student_id.into_inner();
        let sql = format!(
            "SELECT name FROM {STUDENT_TABLE} WHERE student_id = '{}';",
            student_id.inner
        );
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.query_one(&stmt, &[]).await {
            Ok(row) => {
                let name: String = row.get(0);
                HttpResponse::Ok().json(name)
            }
            Err(_) => HttpResponse::NotFound().body("Student not found"),
        }
    }
    
    pub async fn list_all_students(pool: web::Data<Pool>) -> impl Responder {
        let client = pool.get().await.unwrap();
    
        log::debug!("get all students");
        let sql = format!("SELECT name, student_id, email FROM {STUDENT_TABLE}");
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let students = rows
            .into_iter()
            .map(Student::from_row)
            .collect::<Vec<Student>>();
        web::Json(students)
    }    
}

mod post_services {
    use actix_web::{web, HttpResponse, Responder};
    use deadpool_postgres::Pool;

    use crate::model::Student;

    use super::STUDENT_TABLE;

    pub async fn register_student(student: web::Query<Student>, pool: web::Data<Pool>) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("create student");

        let Student { student_id, name, email } = student.into_inner();

        let sql = if let Some(email) = email {
            format!("INSERT INTO {STUDENT_TABLE} (name, student_id, email) VALUES ('{}', '{}', '{}');", name, student_id, email)
        } else {
            format!("INSERT INTO {STUDENT_TABLE} (name, student_id) VALUES ('{}', '{}');", name, student_id)
        };

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[]).await {
            Ok(_) => HttpResponse::Ok().json("Student created"),
            Err(_) => HttpResponse::InternalServerError().body("Error creating student"),
        }
    }
}