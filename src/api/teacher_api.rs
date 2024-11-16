use actix_web::web;

use super::{LIST_ENDPOINT, QUERY_ID_ENDPOINT, REGISTER_ENDPOINT};

pub fn get_teacher_apis() -> actix_web::Scope {
    let query_id_api =
        web::resource(QUERY_ID_ENDPOINT).route(web::get().to(get_services::get_teacher_by_id));

    let list_api =
        web::resource(LIST_ENDPOINT).route(web::get().to(get_services::list_all_teachers));

    let register_api =
        web::resource(REGISTER_ENDPOINT).route(web::post().to(post_services::register_teacher));

    web::scope("/teacher")
        .service(query_id_api)
        .service(list_api)
        .service(register_api)
}

mod get_services {
    use actix_web::{web, HttpResponse, Responder};
    use deadpool_postgres::Pool;

    use crate::model::{QueryById, Teacher};

    use crate::api::TEACHER_TABLE;

    pub async fn get_teacher_by_id(
        teacher_id: web::Query<QueryById>,
        pool: web::Data<Pool>,
    ) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("get teacher by teacher_id");
        let teacher_id = teacher_id.into_inner();
        let sql = format!(
            "SELECT name FROM {TEACHER_TABLE} WHERE teacher_id = '{}';",
            teacher_id.inner
        );
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        match client.query_one(&stmt, &[]).await {
            Ok(row) => {
                let name: String = row.get(0);
                HttpResponse::Ok().json(name)
            }
            Err(_) => HttpResponse::NotFound().body("Teacher not found"),
        }
    }

    pub async fn list_all_teachers(pool: web::Data<Pool>) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("get all teachers");
        let sql = format!("SELECT name, teacher_id, email FROM {TEACHER_TABLE};");
        let stmt = client.prepare(sql.as_str()).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let teachers = rows
            .into_iter()
            .map(Teacher::from_row)
            .collect::<Vec<Teacher>>();
        web::Json(teachers)
    }
}

mod post_services {
    use actix_web::{web, HttpResponse, Responder};
    use deadpool_postgres::Pool;

    use crate::manager::RegexManager;
    use crate::model::Teacher;

    use crate::api::TEACHER_TABLE;

    // TODO: registering teacher requires privileges
    pub async fn register_teacher(
        teacher: web::Query<Teacher>,
        pool: web::Data<Pool>,
        regex: web::Data<RegexManager>,
    ) -> impl Responder {
        let client = pool.get().await.unwrap();

        log::debug!("create teacher");

        if !teacher.check_valid(&regex) {
            return HttpResponse::BadRequest().body("Invalid email or password");
        }

        let Teacher {
            teacher_id,
            name,
            email,
        } = teacher.into_inner();

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
            Ok(_) => HttpResponse::Ok().json("Student created"),
            Err(_) => HttpResponse::InternalServerError().body("Error creating student"),
        }
    }
}
