use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use reqwest::header::HeaderValue;

use crate::http::{
    self,
    message::{Message, MessageConfig},
    request::ChatRequest,
    response::{ChatResponse, Model},
};

use super::{httpclient::HttpClient, structs::AccessToken};

const BASE_URL_AUTH: &str = "https://ngw.devices.sberbank.ru:9443/api";
const BASE_URL: &str = "https://gigachat.devices.sberbank.ru/api";

pub struct ChatClient {
    // Tokens
    basic_token: String,
    auth_token: Option<AccessToken>,

    // Settings for messages
    message_cfg: MessageConfig,
    uuid: String,

    // Other
    httpclient: HttpClient,
}

impl ChatClient {
    pub async fn send_message(&mut self, message: Message) -> anyhow::Result<Message> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            "Content-Type",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append(
            "Accept",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append(
            "Authorization",
            reqwest::header::HeaderValue::from_str(
                format!(
                    "Bearer {}",
                    self.get_auth_token().await.unwrap().access_token
                )
                .as_str(),
            )
            .unwrap(),
        );

        let json_msg = ChatRequest {
            model: self.message_cfg.model.clone(),
            messages: vec![message],
            temperature: self.message_cfg.temperature,
            top_p: self.message_cfg.top_p,
            stream: self.message_cfg.stream,
            max_tokens: self.message_cfg.max_tokens,
            repetition_penalty: self.message_cfg.repetition_penalty,
        };
        
        let resp: ChatResponse = self
            .httpclient
            .post_data(
                &(BASE_URL.to_owned() + "/v1/chat/completions"),
                serde_json::to_string(&json_msg).unwrap(),
                headers,
            )
            .await?;
        
        Ok(resp.choices.last().ok_or_else(|| anyhow!("There is no Choice from the AI"))?.message.clone())
    }

    pub(crate) async fn send_messages(
        &mut self,
        messages: Vec<Message>,
    ) -> anyhow::Result<Message> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            "Content-Type",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append(
            "Accept",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append(
            "Authorization",
            reqwest::header::HeaderValue::from_str(
                format!(
                    "Bearer {}",
                    self.get_auth_token().await.unwrap().access_token
                )
                .as_str(),
            )
            .unwrap(),
        );

        let json_msg = ChatRequest {
            model: self.message_cfg.model.clone(),
            messages: messages,
            temperature: self.message_cfg.temperature,
            top_p: self.message_cfg.top_p,
            stream: self.message_cfg.stream,
            max_tokens: self.message_cfg.max_tokens,
            repetition_penalty: self.message_cfg.repetition_penalty,
        };

        let resp: ChatResponse = self
            .httpclient
            .post_data(
                &(BASE_URL.to_owned() + "/v1/chat/completions"),
                serde_json::to_string(&json_msg).unwrap(),
                headers,
            )
            .await?;
        
        Ok(resp.choices.last().ok_or_else(|| anyhow!("There is no choice from the AI"))?.message.clone())
    }
    pub async fn get_models(&mut self) -> anyhow::Result<Vec<Model>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.append(
            "Authorization",
            HeaderValue::from_str(
                format!(
                    "Bearer {}",
                    self.get_auth_token().await.unwrap().access_token
                )
                .as_str(),
            )
            .unwrap(),
        );

        let resp: serde_json::Value = self
            .httpclient
            .get(&(BASE_URL.to_owned() + "/v1/models"), headers)
            .await?;

        let mdls: Vec<Model> = resp
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|val| serde_json::from_value(val.clone()).unwrap())
            .collect();

        Ok(mdls)
    }

    // Sets to default if new_cfg is None, otherwise set to the passed config
    pub fn reset_msg_config(&mut self, new_cfg: Option<MessageConfig>) {
        self.message_cfg = new_cfg.unwrap_or_default();
    }

    async fn get_auth_token(&mut self) -> anyhow::Result<AccessToken> {
        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() > self.auth_token.as_ref().map_or(0, |tok| tok.expires_at)
        {
            let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
            headers.append(
                "Content-Type",
                reqwest::header::HeaderValue::from_str("application/x-www-form-urlencoded")
                    .unwrap(),
            );
            headers.append(
                "Accept",
                reqwest::header::HeaderValue::from_str("application/json").unwrap(),
            );
            headers.append(
                "RqUID",
                reqwest::header::HeaderValue::from_str(&self.uuid).unwrap(),
            );
            headers.append(
                "Authorization",
                reqwest::header::HeaderValue::from_str(
                    ("Basic ".to_owned() + &self.basic_token).as_str(),
                )
                .unwrap(),
            );
            let mut form_data: HashMap<String, String> = HashMap::new();
            form_data.insert("scope".to_owned(), "GIGACHAT_API_PERS".to_owned());

            let tok: AccessToken = self
                .httpclient
                .post_form(
                    &(BASE_URL_AUTH.to_owned() + "/v2/oauth"),
                    form_data,
                    headers,
                )
                .await
                .expect("Fatal error: Could not get auth token");
            
            self.auth_token = tok.into();
        }

        Ok(self.auth_token.clone().unwrap())
    }
}

pub struct ClientBuilder {
    msg_cfg: Option<MessageConfig>,
    basic_token: Option<String>,
}


impl ClientBuilder {
    pub fn new() -> Self {
        Self { msg_cfg: None, basic_token: None }
    }
    pub fn set_msg_cfg(mut self, msg_cfg: MessageConfig) -> Self {
        self.msg_cfg = msg_cfg.into();
        self
    }
    pub fn set_basic_token(mut self, basic_token: &str) -> Self {
        self.basic_token = basic_token.to_owned().into();
        self
    }
    pub fn build(self) -> ChatClient {
        ChatClient { basic_token: self.basic_token.expect("Token must be set"), 
        auth_token: None, message_cfg: self.msg_cfg.unwrap_or_default(), uuid: uuid::Uuid::new_v4().to_string(), httpclient: HttpClient::new() }
    }
}