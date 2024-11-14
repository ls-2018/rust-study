#![allow(unused)]
use anyhow::Result;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{Html, Sse};
use axum::{Router, extract::State, routing::get};
use futures::StreamExt;
use reqwest_eventsource::{Event as REvent, EventSource};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_stream::Stream;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

struct AppState {
    tx: broadcast::Sender<Event>,
}

async fn send_message(State(app_state): State<Arc<AppState>>) -> Html<&'static str> {
    app_state.tx.send(Event::default().data("custom_data")).expect("send message");

    Html("<h1>Hello, World!</h1>")
}

async fn sse_handler(State(app_state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = app_state.tx.subscribe();

    let mystream = BroadcastStream::new(rx).map(|x| {
        let binding = x.clone();
        let res = binding.as_ref();
        println!("--->{:?}", res);
        x
    });

    Sse::new(mystream).keep_alive(KeepAlive::new().interval(Duration::from_secs(1)).text("keep-alive-text"))
}

async fn main() -> Result<()> {
    let (tx, _) = broadcast::channel::<Event>(100);
    let app_state = Arc::new(AppState { tx });
    let app = Router::new()
        .route("/send_message", get(send_message))
        .route("/sse", get(sse_handler))
        .with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
//
// async fn custom() {
//     let mut es = EventSource::get(format!("http://0.0.0.0:3000/see?token={}", "token"));
//     tokio::spawn(async move {
//         while let Some(event) = es.next().await {
//             match event {
//                 Ok(REvent::Open) => println!("Connection Open!"),
//                 Ok(REvent::Message(message)) => match message.event.as_str() {
//                     v => {
//                         println!("expected event: {:?}", v);
//                     }
//                 },
//                 Err(err) => {
//                     println!("Error: {}", err);
//                     es.close();
//                 }
//             }
//         }
//     });
// }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn entry() {
        let f1 = main();
        // let f2 = custom();
        // join!(f1,f2);
        f1.await.unwrap();
    }
}
