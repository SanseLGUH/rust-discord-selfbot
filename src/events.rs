use crate::models::{Message, User, Guild, Channel, Presence};
use crate::http::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, http: Arc<HttpClient>, user: User) {}

    async fn message_create(&self, http: Arc<HttpClient>, message: Message) {}

    async fn message_update(&self, http: Arc<HttpClient>, old_message: Option<Message>, new_message: Message) {}

    async fn message_delete(&self, http: Arc<HttpClient>, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake) {}

    async fn guild_create(&self, http: Arc<HttpClient>, guild: Guild) {}

    async fn guild_delete(&self, http: Arc<HttpClient>, guild_id: crate::utils::Snowflake) {}

    async fn channel_create(&self, http: Arc<HttpClient>, channel: Channel) {}

    async fn channel_delete(&self, http: Arc<HttpClient>, channel: Channel) {}

    async fn presence_update(&self, http: Arc<HttpClient>, presence: Presence) {}

    async fn typing_start(&self, http: Arc<HttpClient>, channel_id: crate::utils::Snowflake, user_id: crate::utils::Snowflake) {}

    async fn poll_vote_add(&self, http: Arc<HttpClient>, user_id: crate::utils::Snowflake, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake, answer_id: u32) {}

    async fn poll_vote_remove(&self, http: Arc<HttpClient>, user_id: crate::utils::Snowflake, channel_id: crate::utils::Snowflake, message_id: crate::utils::Snowflake, answer_id: u32) {}

    async fn raw(&self, _event: serde_json::Value) {}
}

pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {}