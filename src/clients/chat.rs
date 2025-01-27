use anyhow::anyhow;

use crate::http::message::Message;

use super::client::ChatClient;

pub struct Chat {
    client: ChatClient,
    message_history: Vec<Message>,
}

impl Chat {
    pub fn new(client: ChatClient) -> Self {
        Self {
            client: client,
            message_history: Vec::new(),
        }
    }
    pub fn get_client_mut(&mut self) -> &mut ChatClient {
        &mut self.client
    }
    pub fn get_client(&self) -> &ChatClient {
        &self.client
    }
    pub async fn send_message(&mut self, message: Message) -> anyhow::Result<Message> {
        self.message_history.push(message.clone());
        let resp = self
            .client
            .send_messages(self.message_history.clone())
            .await?;
        
        self.message_history.push(resp.clone());
        Ok(resp)
    }
    pub fn get_message_history(&self) -> &Vec<Message> {
        &self.message_history
    }
}
