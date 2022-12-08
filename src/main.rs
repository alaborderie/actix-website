use actix_files::Files;
use actix_web::HttpRequest;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

use controllers::contact_controller;
use controllers::home_controller;
use models::ContactForm;

mod controllers;
mod models;

#[get("/")]
async fn home(_req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    home_controller(hb).await
}

#[post("/contact")]
async fn contact(_req: HttpRequest, contact_form: web::Json<ContactForm>) -> HttpResponse {
    contact_controller(contact_form.into_inner()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Init Templating engine
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // Server config
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(Files::new("/static", "static").show_files_listing())
            .service(home)
            .service(contact)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
