use discord_selfbot::{ClientBuilder, EventHandler, User, Message};
use discord_selfbot::http::HttpClient;
use std::sync::Arc;
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl EventHandler for MyEventHandler {
    async fn ready(&self, _http: Arc<HttpClient>, user: User) {
        println!("{} is ready!", user.username);
    }

    async fn message_create(&self, http: Arc<HttpClient>, message: Message) {
        if message.content == "ping" {
            if let Err(e) = message.reply(&http, "pong").await {
                eprintln!("Failed to reply: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(MyEventHandler);
    let client = ClientBuilder::new("token")
        .event_handler(handler)
        .build()
        .await?;
    client.listen().await?;
    Ok(())
}