use actix_files::{Files, NamedFile};
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use handlebars::Handlebars;
use serde_json::json;
use std::env;

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

#[get("/robots.txt")]
async fn robots_txt() -> impl Responder {
    NamedFile::open_async("./static/robots.txt").await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({});
            let body = hb.render("404", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
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
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .app_data(handlebars_ref.clone())
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(Files::new("/static", "./static/"))
            .service(home)
            .service(contact)
            .service(robots_txt)
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
