#![allow(unused)]
use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use http::{HeaderMap, StatusCode, header::LOCATION};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: MySqlPool,
}

#[derive(Debug, FromRow)]
#[sqlx(type_name = "urls", rename_all = "snake_case")]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}

const LISTEN_ADDR: &str = "127.0.0.1:8000";

async fn shorten(State(state): State<AppState>, Json(data): Json<ShortenReq>) -> Result<impl IntoResponse, StatusCode> {
    let id = state.shorten(&data.url).await.map_err(|e| {
        warn!("Failed to shorten URL: {e}");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;
    let body = Json(ShortenRes {
        url: format!("http://{}/{}", LISTEN_ADDR, id),
    });
    Ok((StatusCode::CREATED, body))
}

async fn redirect(Path(id): Path<String>, State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let url = state.get_url(&id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}

impl AppState {
    async fn try_new(url: &str) -> Result<Self> {
        let pool = MySqlPool::connect(url).await?;
        // Create table if not exists
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS urls (
              `id` char(6) NOT NULL,
              `url` text NOT NULL,
              PRIMARY KEY (`id`),
              INDEX `ui`(`url`(20))
            )
            "#,
        )
        .execute(&pool)
        .await?;
        Ok(Self { db: pool })
    }

    async fn shorten(&self, url: &str) -> Result<String> {
        let id = nanoid!(6);
        sqlx::query(
            "INSERT INTO urls (id, url) VALUES (?,?) ON DUPLICATE KEY UPDATE url = VALUES(url)",
            // "INSERT INTO urls (id, url) VALUES ($1, $2) ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id",
        )
        .bind(&id)
        .bind(url)
        .execute(&self.db)
        .await?;
        let ret: UrlRecord = sqlx::query_as("SELECT * FROM urls where id = ?").bind(&id).fetch_one(&self.db).await?;
        Ok(ret.id)
    }

    async fn get_url(&self, id: &str) -> Result<String> {
        let ret: UrlRecord = sqlx::query_as("SELECT url FROM urls WHERE id = ?").bind(&id).fetch_one(&self.db).await?;
        Ok(ret.url)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let url = "mysql://root:root@localhost:3306/shortener";
    let state = AppState::try_new(url).await?;
    info!("Connected to database: {url}");
    let listener = TcpListener::bind(LISTEN_ADDR).await?;
    info!("Listening on: {}", LISTEN_ADDR);

    let app = Router::new().route("/", post(shorten)).route("/:id", get(redirect)).with_state(state);

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
