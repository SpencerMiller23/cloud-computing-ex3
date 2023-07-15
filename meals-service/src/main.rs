mod api;

use api::dish::{
    rebuild, // Delete
    get_dishes,
    create_dish,
    delete_dishes,
    get_dish,
    delete_dish
};

use api::meal::{
    create_meal,
    get_meals,
    delete_meals,
    get_meal,
    delete_meal,
    update_meal
};

mod repository;

use repository::state::AppState;

use actix_web::{
    get,
    HttpServer,
    App,
    web::Data,
    middleware::Logger,
    Responder,
    HttpResponse
};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_state = Data::new(AppState::new());

    // Boilerplate code taken from Actix Web docs: https://actix.rs/docs/getting-started
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(app_state.clone())
            .service(health_check)
            .service(get_dishes)
            .service(create_dish)
            .service(delete_dishes)
            .service(get_dish)
            .service(delete_dish)
            .service(create_meal)
            .service(get_meals)
            .service(delete_meals)
            .service(get_meal)
            .service(delete_meal)
            .service(update_meal)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
