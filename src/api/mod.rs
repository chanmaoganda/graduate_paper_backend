mod full_paper_api;
mod paper_api;
mod student_api;
mod teacher_api;
mod login_api;

use actix_web::web;

const QUERY_ID_ENDPOINT: &str = "/query/{id}";
const LIST_ENDPOINT: &str = "/list";

const PAPER_TABLE: &str = env!("PAPER_TABLE");
const STUDENT_TABLE: &str = env!("STUDENT_TABLE");
const TEACHER_TABLE: &str = env!("TEACHER_TABLE");

pub fn configure_apis(cfg: &mut web::ServiceConfig) {
    let student_scope = student_api::get_student_apis();
    let teacher_scope = teacher_api::get_teacher_apis();
    let paper_scope = paper_api::get_paper_apis();
    let full_paper_scope = full_paper_api::get_full_paper_apis();
    let login_service = login_api::get_login_api();

    let api_scope = web::scope("/api")
        .service(student_scope)
        .service(teacher_scope)
        .service(paper_scope)
        .service(full_paper_scope);

    cfg.service(api_scope)
       .service(login_service);
}
