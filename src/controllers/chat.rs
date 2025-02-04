use uuid::Uuid;

use crate::http::message::Message;

use super::client::GigaClient;

/// Used to make a chat, stores message history, makes caching messages possible, so a response comes faster
pub struct Chat {
    client: GigaClient,
    message_history: Vec<Message>,
    cache_uuid: String,
}

impl Chat {
    /// Create a non-cached version of chat
    pub fn new(client: GigaClient) -> Self {
        Self {
            client,
            message_history: Vec::new(),
            cache_uuid: String::new(),
        }
    }
    /// Create a cached version of chat
    pub fn new_cached(client: GigaClient) -> Self {
        let cache = Uuid::new_v4().to_string();
        Self {
            client,
            message_history: Vec::new(),
            cache_uuid: cache,
        }
    }

    /// Returns a mutable client, which can be used to interace with files, get available models and etc..
    pub fn get_client_mut(&mut self) -> &mut GigaClient {
        &mut self.client
    }

    /// Returns a immutable client, which can be used for something, i don't know what exactly
    pub fn get_client(&self) -> &GigaClient {
        &self.client
    }

    /// Sends a message and stores it in the message history
    pub async fn send_message(&mut self, message: Message) -> anyhow::Result<Message> {
        self.message_history.push(message.clone());

        let resp = self
            .client
            .send_messages(
                self.message_history.clone(),
                if self.cache_uuid.is_empty() {
                    None
                } else {
                    Some(&self.cache_uuid)
                },
            )
            .await?;
        self.message_history.push(resp.clone());
        Ok(resp)
    }

    // Returns a reference to the message history, allowing read-only access
    pub fn get_message_history(&self) -> &Vec<Message> {
        &self.message_history
    }
}
