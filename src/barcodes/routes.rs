use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    surname: String,
}

#[get("/{id}")]
async fn barcode(path: web::Path<u64>) -> Result<impl Responder> {
    let user = User {
        id: path.into_inner(),
        name: "John Doe".to_string(),
        surname: "Smith".to_string(),
    };

    Ok(web::Json(user))
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/barcodes").service(barcode));
}
