use crate::routes::{handlers::post_handler, middleware};
use actix_web::{middleware::from_fn, web};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("secure/post")
                .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
                .service(post_handler::create_post)
                .service(post_handler::my_posts),
        )
        .service(
            web::scope("/post")
                .service(post_handler::all_posts)
                .service(post_handler::single_post),
        );
}
