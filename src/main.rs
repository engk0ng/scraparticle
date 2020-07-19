mod config;
mod handlers;
mod errors;
mod models;
mod scrap;

use crate::config::Config;
use crate::handlers::*;
use crate::models::AppState;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use slog::info;
use std::io;
use std::env;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let host = "127.0.0.1".to_string();
    let port = "8989".to_string();
    let log = Config::configure_log();

    HttpServer::new(move || {
        App::new()
        .data(AppState{log: log.clone()
        })
        .wrap(middleware::Logger::default())
        .route("/", web::get().to(status))
        .route("/asysyariah", web::get().to(asysyariah))
        .route("/nasehat", web::get().to(nasehat))
        .route("/akhbar", web::get().to(akhbar))
        .route("/bnpb", web::get().to(bpnb))
        .route("/ciamis", web::get().to(ciamis))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
