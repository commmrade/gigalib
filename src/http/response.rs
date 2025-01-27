use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub owned_by: String,

    #[serde(alias = "type")]
    pub type_: String,
}
