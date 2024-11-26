use actix_web::web;

use super::{LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT, UNREGISTER_ENDPOINT};

pub fn get_student_apis() -> actix_web::Scope {
    let query_id_api =
        web::resource(QUERY_ID_ENDPOINT).route(web::get().to(get_services::get_student_by_id));

    let list_api =
        web::resource(LIST_ENDPOINT).route(web::get().to(get_services::list_all_students));

    let register_api =
        web::resource(REGISTER_ENDPOINT).route(web::post().to(post_services::register_student));

    let unregister_api =
        web::resource(UNREGISTER_ENDPOINT).route(web::post().to(post_services::unregister_student));

    web::scope("/student")
        .service(query_id_api)
        .service(list_api)
        .service(register_api)
        .service(unregister_api)
}

mod get_services {
    use crate::model::{QueryById, Student};
    use actix_web::{web, HttpResponse, Responder};
    use deadpool_postgres::Pool;

    use crate::api::STUDENT_TABLE;

    pub async fn get_student_by_id(
        student_id: web::Query<QueryById>,
        pool: web::Data<Pool>,
    ) -> impl Responder {
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
                HttpResponse::Ok().body(name)
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

    use crate::manager::RegexManager;
    use crate::model::Student;

    use crate::api::STUDENT_TABLE;

    pub async fn register_student(
        student: web::Query<Student>,
        pool: web::Data<Pool>,
        regex: web::Data<RegexManager>,
    ) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("register student");

        if !student.check_valid(&regex) {
            return HttpResponse::BadRequest().body("Invalid email or password");
        }

        let Student {
            student_id,
            name,
            email,
        } = student.into_inner();

        let sql = if let Some(email) = email {
            format!(
                "INSERT INTO {STUDENT_TABLE} (name, student_id, email) VALUES ('{}', '{}', '{}');",
                name, student_id, email
            )
        } else {
            format!(
                "INSERT INTO {STUDENT_TABLE} (name, student_id) VALUES ('{}', '{}');",
                name, student_id
            )
        };

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[]).await {
            Ok(_) => HttpResponse::Ok().body("Student created"),
            Err(_) => HttpResponse::InternalServerError().body("Error creating student"),
        }
    }

    pub async fn unregister_student(
        student: web::Query<Student>,
        pool: web::Data<Pool>,
    ) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("unregister student");

        let Student {
            student_id, name, ..
        } = student.into_inner();

        let sql = format!(
            "DELETE FROM {STUDENT_TABLE} WHERE student_id = '{}' AND name = '{}';",
            student_id, name
        );

        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.execute(&stmt, &[]).await {
            Ok(_) => HttpResponse::Ok().body("Student unregistered"),
            Err(_) => HttpResponse::InternalServerError().body("Error unregistering student"),
        }
    }
}
