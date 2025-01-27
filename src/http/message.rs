use serde::{Deserialize, Serialize};

pub enum Role {
    User,
    Assistant,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Assistant => "assistant".to_owned(),
            Role::User => "user".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    pub role: String,
}

impl Message {
    pub fn new(content: &str, role: Role) -> Self {
        Self {
            content: content.to_owned(),
            role: role.to_string(),
        }
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Self {
            content: value,
            role: "user".to_owned(),
        }
    }
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self {
            content: value.to_owned(),
            role: Role::User.to_string(),
        }
    }
}
impl From<(&str, Role)> for Message {
    fn from(value: (&str, Role)) -> Self {
        Self {
            content: value.0.to_owned(),
            role: value.1.to_string(),
        }
    }
}
impl From<(String, Role)> for Message {
    fn from(value: (String, Role)) -> Self {
        Self {
            content: value.0,
            role: value.1.to_string(),
        }
    }
}
