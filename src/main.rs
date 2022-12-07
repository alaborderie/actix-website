use actix_files::Files;
use actix_web::HttpRequest;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
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
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .service(home)
            .service(contact)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
