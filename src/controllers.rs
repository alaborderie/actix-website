use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

use crate::models::{ContactForm, JsonResponse};

pub async fn home_controller(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({});
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn contact_controller(contact_form: ContactForm) -> HttpResponse {
    print!(
        "{} {} {}",
        contact_form.name, contact_form.email, contact_form.message
    );
    if contact_form.name.is_empty()
        || contact_form.email.is_empty()
        || contact_form.message.is_empty()
    {
        let error_response = JsonResponse {
            message: "All fields are required.",
        };
        return HttpResponse::BadRequest().json(web::Json(error_response));
    }
    let response = JsonResponse {
        message: "Email sent.",
    };
    HttpResponse::Ok().json(web::Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_web::test]
    async fn test_contact_ok() {
        let data = ContactForm {
            name: "Foo Bar".to_string(),
            email: "foo@bar.com".to_string(),
            message: "This is a test!".to_string(),
        };
        let resp = contact_controller(data).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_contact_not_ok() {
        let data = ContactForm {
            name: "".to_string(),
            email: "".to_string(),
            message: "".to_string(),
        };
        let resp = contact_controller(data).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
