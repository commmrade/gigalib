use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::User => {
                write!(f, "user")
            }
            Self::Assistant => {
                write!(f, "assistant")
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

impl Message {
    pub fn new(content: &str, role: Role) -> Self {
        Self {
            content: content.to_owned(),
            role: role,
        }
    }
    pub fn from_str(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            role: Role::User,
        }
    }
    pub fn from_tuple(contents: &(&str, Role)) -> Self {
        Self {
            content: contents.0.to_owned(),
            role: Role::User,
        }
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self {
            content: value,
            role: Role::User,
        }
    }
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self {
            content: value.to_owned(),
            role: Role::User,
        }
    }
}
impl From<(&str, Role)> for Message {
    fn from(value: (&str, Role)) -> Self {
        Self {
            content: value.0.to_owned(),
            role: value.1,
        }
    }
}
impl From<(String, Role)> for Message {
    fn from(value: (String, Role)) -> Self {
        Self {
            content: value.0,
            role: value.1,
        }
    }
}

#[derive(Clone)]
pub struct MessageConfig {
    pub model: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>, // 0..1
    pub stream: Option<bool>,
    pub max_tokens: Option<u32>,
    pub repetition_penalty: Option<f32>,
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            model: "GigaChat".to_owned(),
            temperature: None,
            top_p: None,
            stream: None,
            max_tokens: None,
            repetition_penalty: None,
        }
    }
}

#[derive(Default)]
pub struct MessageConfigBuilder {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>, // 0..1
    pub stream: Option<bool>,
    pub max_tokens: Option<u32>,
    pub repetition_penalty: Option<f32>,
}

impl MessageConfigBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            temperature: None,
            top_p: None,
            stream: None,
            max_tokens: None,
            repetition_penalty: None,
        }
    }
    pub fn set_model(mut self, model: &str) -> Self {
        self.model = Some(model.to_owned());
        self
    }
    pub fn set_temp(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
    pub fn set_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }
    pub fn set_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    pub fn set_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }
    pub fn set_rep_penalty(mut self, penalty: f32) -> Self {
        self.repetition_penalty = Some(penalty);
        self
    }
    pub fn build(&self) -> MessageConfig {
        MessageConfig {
            model: self
                .model
                .as_ref()
                .expect("Model should be set")
                .to_string(),
            temperature: self.temperature,
            top_p: self.top_p,
            stream: self.stream,
            max_tokens: self.max_tokens,
            repetition_penalty: self.repetition_penalty,
        }
    }
}
