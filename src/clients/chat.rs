use anyhow::anyhow;
use uuid::Uuid;

use crate::http::message::Message;

use super::client::GigaClient;

pub struct Chat {
    client: GigaClient,
    message_history: Vec<Message>,
    cache_uuid: String,
}

impl Chat {
    pub fn new(client: GigaClient) -> Self {
        Self {
            client: client,
            message_history: Vec::new(),
            cache_uuid: String::new(),
        }
    }
    pub fn new_cached(client: GigaClient) -> Self {
        let cache = Uuid::new_v4().to_string();

        Self {
            client: client,
            message_history: Vec::new(),
            cache_uuid: cache,
        }
    }

    pub fn get_client_mut(&mut self) -> &mut GigaClient {
        &mut self.client
    }
    pub fn get_client(&self) -> &GigaClient {
        &self.client
    }
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
    pub fn get_message_history(&self) -> &Vec<Message> {
        &self.message_history
    }
}
