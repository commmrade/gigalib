use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use reqwest::header::HeaderValue;

use crate::http::{
    self,
    message::Message,
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
    max_tokens: u32,
    model: String,
    uuid: String,
    repet_penalty: f32,

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
            model: self.model.clone(),
            messages: vec![message],
            stream: false,
            max_tokens: self.max_tokens,
            repetition_penalty: self.repet_penalty,
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
            model: self.model.clone(),
            messages: messages,
            stream: false,
            max_tokens: self.max_tokens,
            repetition_penalty: self.repet_penalty,
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

    pub fn set_model(&mut self, model_name: &str) {
        self.model = model_name.to_owned();
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

pub struct ChatClientBuilder {
    basic_token: Option<String>,
    max_tokens_per_msg: Option<u32>,
    model: Option<String>,
    repet_penalty: Option<f32>,
}

impl ChatClientBuilder {
    pub fn new() -> Self {
        ChatClientBuilder {
            basic_token: None,
            max_tokens_per_msg: None,
            model: None,
            repet_penalty: None,
        }
    }
    pub fn with_env() -> Self {
        let basic_token: String =
            std::env::var("GIGACHAT_TOKEN").expect("$GIGACHAT_TOKEN must be set");
        ChatClientBuilder {
            basic_token: basic_token.into(),
            max_tokens_per_msg: None,
            model: None,
            repet_penalty: None,
        }
    }

    pub fn set_token(mut self, token: &str) -> Self {
        self.basic_token = Some(token.to_string());
        self
    }

    pub fn set_max_tokens(mut self, num: u32) -> ChatClientBuilder {
        self.max_tokens_per_msg = num.into();
        self
    }
    pub fn set_model(mut self, model: &str) -> ChatClientBuilder {
        self.model = model.to_owned().into();
        self
    }
    pub fn set_repet_penalty(mut self, penalty: f32) -> ChatClientBuilder {
        self.repet_penalty = penalty.into();
        self
    }

    pub fn build(&self) -> ChatClient {
        ChatClient {
            basic_token: self.basic_token.clone().expect("Basic token must be set!"),
            auth_token: None,
            max_tokens: self.max_tokens_per_msg.unwrap_or(9999),
            model: self.model.clone().unwrap_or("GigaChat".to_owned()),
            httpclient: HttpClient::new(),
            uuid: uuid::Uuid::new_v4().to_string(),
            repet_penalty: self.repet_penalty.unwrap_or(1.0),
        }
    }
}
