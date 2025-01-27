use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    pub max_tokens: u32,
    pub repetition_penalty: f32,
}
