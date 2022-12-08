use actix_web::{web, HttpResponse};
use discord_webhook::client::WebhookClient;
use handlebars::Handlebars;
use serde_json::json;
use std::env;
use uuid::Uuid;

use crate::models::{ContactForm, JsonResponse};

pub async fn home_controller(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let id = Uuid::new_v4();
    let data = json!({ "uuid": format!("{}", id) });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn contact_controller(contact_form: ContactForm) -> HttpResponse {
    if contact_form.name.is_empty()
        || contact_form.email.is_empty()
        || contact_form.message.is_empty()
    {
        let error_response = JsonResponse {
            message: "All fields are required.",
        };
        return HttpResponse::BadRequest().json(web::Json(error_response));
    }
    if !cfg!(test) {
        let url = env::var("DISCORD_HOOK_URL").unwrap();
        let client = WebhookClient::new(&url);
        let _webhook_response = client
            .send(|message| {
                message
                    .content(&format!(
                        "Formulaire de contact reçu :

- Nom: {}
- Email: {}
- Message: {}",
                        contact_form.name, contact_form.email, contact_form.message
                    ))
                    .username("Site antoinelaborderie.com")
            })
            .await;
    }
    let response = JsonResponse {
        message: "Webhook sent.",
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
