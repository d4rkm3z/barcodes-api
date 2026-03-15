mod barcodes;
mod config;

use crate::barcodes::routes as barcodes_routes;

use crate::config::{load_config, Config};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use sea_orm::Database;

pub struct AppState {
    config: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config().unwrap();

    let state = web::Data::new(AppState {
        config
    });
    let conn_db = Database::connect(&state.config.database.connection_string).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(conn_db.clone())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(web::scope("/api").configure(barcodes_routes::config))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
