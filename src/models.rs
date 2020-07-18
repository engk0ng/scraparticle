use serde::{Deserialize, Serialize};
use slog;

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
}

#[derive(Debug, Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub judul: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct Rest {
    pub code: i32,
    pub status: String,
    pub data: Option<Vec<Article>>,
}

#[derive(Debug, Serialize)]
pub struct Ciamis {
    pub code: i32,
    pub status: String,
    pub data: Option<Vec<String>>,
}