#![allow(unused)]

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    routing::{get, patch},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tracing::{info, instrument};
use tracing_subscriber::{Layer as _, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    age: Option<u8>,
    skills: Option<Vec<String>>,
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    (*user.lock().unwrap()).clone().into()
}

#[instrument]
async fn update_handler(State(user): State<Arc<Mutex<User>>>, Json(user_update): Json<UserUpdate>) -> Json<User> {
    let mut user = user.lock().unwrap();
    if let Some(age) = user_update.age {
        user.age = age;
    }
    if let Some(skills) = user_update.skills {
        user.skills = skills;
    }
    (*user).clone().into()
}
#[tokio::main]
async fn main() -> Result<()> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    let app = Router::new().route("/", get(user_handler)).route("/", patch(update_handler)).with_state(user);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[tokio::test]
    async fn entry() -> Result<()> {
        main()
    }
}
