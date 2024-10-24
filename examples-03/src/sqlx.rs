#![allow(unused)]
use futures::StreamExt;
use sqlx::postgres::PgListener;
use sqlx::Error;
use tracing::{info, warn};

async fn main() -> anyhow::Result<()> {
    let mut listener = PgListener::connect("").await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;
    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(Ok(notif)) = stream.next().await {
            info!("Received notification: {:?}", notif);
            let _type = notif.channel();
            match _type {
                "chat_updated" => {
                    println!("{}", notif.payload());
                }
                "chat_message_created" => {}
                _ => {}
            }
        }
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
