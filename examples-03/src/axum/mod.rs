use serde_with::serde_derive::{Deserialize, Serialize};

mod axum_study;
mod layer;
mod middwares;
mod opentelemetry_otlp;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub ws_name: String,
    pub fullname: String,
    pub email: String,
    pub password_hash: Option<String>,
}

impl User {
    pub(crate) fn new(p0: i32, p1: &str, p2: &str) -> Self {
        todo!()
    }
}
