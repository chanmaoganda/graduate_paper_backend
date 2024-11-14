mod student;

use actix_web::web;

pub fn configure_apis(cfg: &mut web::ServiceConfig) {
    let routes = vec![
        web::resource("/student/{id}").route(web::get().to(student::get_student_by_id))
    ];

    for route in routes {
        cfg.service(web::scope("/api").service(route));
    }
}