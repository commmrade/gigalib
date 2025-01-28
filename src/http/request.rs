use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // 0..1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
}
