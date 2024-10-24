use anyhow::Result;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{Html, Sse};
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

struct AppState {
    tx: broadcast::Sender<Event>,
    tx_as_string: broadcast::Sender<String>,
}

async fn send_message(State(app_state): State<Arc<AppState>>) -> Html<&'static str> {
    app_state
        .tx
        .send(Event::default().data("custom_data"))
        .expect("send message");

    Html("<h1>Hello, World!</h1>")
}

async fn sse_handler(
    State(app_state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = app_state.tx.subscribe();

    let mystream = BroadcastStream::new(rx);

    Sse::new(mystream).keep_alive(KeepAlive::default())
}

async fn send_message_as_string(State(app_state): State<Arc<AppState>>) -> Html<&'static str> {
    app_state
        .tx_as_string
        .send("custom_data".to_string())
        .expect("send message");

    Html("<h1>Hello, World!</h1>")
}

async fn sse_handler_as_string(
    State(app_state): State<Arc<AppState>>,
    // ) -> Sse<impl Stream<Item=Result<String, BroadcastStreamRecvError>>> {
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = app_state.tx_as_string.subscribe();

    let mystream = BroadcastStream::new(rx).map(|s| Ok(Event::default().data(&s.unwrap())));

    Sse::new(mystream).keep_alive(KeepAlive::default())
}

async fn main() -> Result<()> {
    let (tx, _) = broadcast::channel::<Event>(100);
    let (tx_as_string, _) = broadcast::channel::<String>(100);
    let app_state = Arc::new(AppState { tx, tx_as_string });
    let app = Router::new()
        .route("/send_message", get(send_message))
        .route("/sse", get(sse_handler))
        .route("/send_message_as_string", get(send_message_as_string))
        .route("/sse_as_string", get(sse_handler_as_string))
        .with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn entry() -> Result<()> {
        main().await
    }
}
