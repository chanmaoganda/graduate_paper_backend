mod paper_api;
mod student_api;
mod teacher_api;

use actix_web::web;

pub fn configure_apis(cfg: &mut web::ServiceConfig) {
    let student_scope = student_api::get_student_apis();
    let teacher_scope = teacher_api::get_teacher_apis();
    let paper_scope = paper_api::get_paper_apis();

    let api_scope = web::scope("/api")
        .service(student_scope)
        .service(teacher_scope)
        .service(paper_scope)
    ;

    cfg
        .service(api_scope);
}
