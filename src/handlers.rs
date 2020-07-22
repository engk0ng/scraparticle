
use crate::scrap;
use crate::models::{AppState, Status};

use crate::errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use slog::{error, o, Logger};

fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let log = log.new(o!(
            "cause" => err.cause.clone()
        ));
        error!(log, "{}", err.message());
        err
    }
}

pub async fn status() -> Result<impl Responder, AppError> {
    Ok(web::HttpResponse::Ok().json(Status{
        status: "Up".to_string(),
    }))
}

pub async fn asysyariah(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "asysyariah"));
    let result = scrap::get_article_asy_syariah(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}

pub async fn nasehat(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "nasehat"));
    let result = scrap::get_article_nasehat(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}

pub async fn akhbar(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "akhbar"));
    let result = scrap::get_article_akhbar(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}

pub async fn bpnb(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "bpnb"));
    let result = scrap::get_article_bnpb(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}

pub async fn ciamis(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "ciamis"));
    let result = scrap::get_data_ciamis(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}

pub async fn get_article_covig_gov(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let sublog = state.log.new(o!("handler" => "get_article_covig_go"));
    let result = scrap::get_news_covid_gov(sublog.clone()).await;
    result
    .map(|res| HttpResponse::Ok().json(res))
    .map_err(log_error(sublog))
}
