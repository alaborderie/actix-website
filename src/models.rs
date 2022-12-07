use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
    pub message: &'static str,
}
