use actix_web::{web, Scope};

pub fn task_mng_routes() -> Scope {
    web::scope("/app")
        // .service(web::scope("/user")
        //     .route("/create", route))
        // .service(web::scope("/task")
        //     .route("/create", route))
}
