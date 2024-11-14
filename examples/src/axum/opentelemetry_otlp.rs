#![allow(unused)]

use axum::extract::{FromRequestParts, Path};
use axum::response::IntoResponse;
use axum::{Router, extract::Request, routing::get};
use http::header::LOCATION;
use http::{HeaderMap, Method, StatusCode};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource, runtime,
    trace::{self, RandomIdGenerator, Tracer},
};
use std::time::Duration;
use tokio::{
    join,
    net::TcpListener,
    time::{Instant, sleep},
};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[instrument(fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
async fn index_handler(req: Request) -> String {
    let (parts, body) = req.into_parts();
    assert_eq!(parts.method, Method::GET);

    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status_code = 200, "index handler completed");
    ret
}

async fn index_handler_id(Path(id): Path<String>) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, "url".parse().unwrap());
    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}

#[instrument]
async fn long_task() -> String {
    let start = Instant::now();
    let sl = sleep(Duration::from_millis(11));
    // spawn multiple tasks

    let t1 = task1();
    let t2 = task2();
    let t3 = task3();
    join!(sl, t1, t2, t3);
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");
    let res = serde_json::to_string(&elapsed).expect("--> time");
    res
}

#[instrument]
async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

#[instrument]
async fn task2() {
    sleep(Duration::from_millis(50)).await;
}

#[instrument]
async fn task3() {
    sleep(Duration::from_millis(30)).await;
}

pub fn init_tracer() -> anyhow::Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_endpoint("http://localhost:4317"))
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(32)
                .with_max_attributes_per_span(64)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "axum-tracing")])),
        )
        .install_batch(runtime::Tokio)?;
    Ok(tracer)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // console layer for tracing-subscriber
    let console = fmt::Layer::new().with_span_events(FmtSpan::CLOSE).pretty().with_filter(LevelFilter::DEBUG);

    // file appender layer for tracing-subscriber
    let file_appender = tracing_appender::rolling::daily("/tmp/logs", "ecosystem.log");
    let (non_blocking, _) = tracing_appender::non_blocking(file_appender);
    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        // .pretty()
        .with_filter(LevelFilter::INFO);

    // opentelemetry tracing layer for tracing-subscriber
    let tracer = init_tracer()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry().with(console).with(file).with(opentelemetry).init();

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler)).route("/:id", get(index_handler_id));

    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[tokio::test]
    async fn entry() -> anyhow::Result<()> {
        main()
    }
}
