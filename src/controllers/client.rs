use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    multipart::{Form, Part},
};

use crate::http::{
    message::{Message, MessageConfig},
    request::ChatRequest,
    response::{ChatResponse, Model},
};

use super::{access_token::AccessToken, file::GigaFile, httpclient::HttpClient};

const BASE_URL_AUTH: &str = "https://ngw.devices.sberbank.ru:9443/api";
const BASE_URL: &str = "https://gigachat.devices.sberbank.ru/api";

/// The main thing, which interacts with the GigaChat API
#[derive(Clone)]
pub struct GigaClient {
    // Tokens
    basic_token: String,
    auth_token: Arc<Option<AccessToken>>,

    // Settings for messages
    message_cfg: MessageConfig,
    uuid: String,

    // Other
    httpclient: HttpClient,
}

// impl Clone for GigaClient {
//     fn clone(&self) -> Self {
//         Self {
//             basic_token: self.basic_token.clone(),
//             auth_token: Arc::new(Mutex::new),
//             message_cfg: (),
//             uuid: (),
//             httpclient: (),
//         }
//     }
// }

impl GigaClient {
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

        Ok(resp
            .choices
            .last()
            .ok_or_else(|| anyhow!("There is no Choice from the AI"))?
            .message
            .clone())
    }

    /// Non-pub function used for sending multiple messages, primarily used by 'Chat'
    pub(crate) async fn send_messages(
        &mut self,
        messages: Vec<Message>,
        cache_uuid: Option<&str>,
    ) -> anyhow::Result<Message> {
        let mut headers = HeaderMap::new();
        headers.append(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());
        if let Some(cache_str) = cache_uuid {
            headers.append("X-Session-ID", HeaderValue::from_str(cache_str).unwrap());
        }
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
            messages,
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

        Ok(resp
            .choices
            .last()
            .ok_or_else(|| anyhow!("There is no choice from the AI"))?
            .message
            .clone())
    }

    /// Returns available GigaChat AI models
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

    /// Returns file information, which includes timestamps, filename, id and etc...
    pub async fn get_file_info(&mut self, file_id: &str) -> anyhow::Result<GigaFile> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
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
        let api_url = BASE_URL.to_owned() + &format!("/v1/files/{}", file_id);
        let resp: GigaFile = self.httpclient.get(&api_url, headers).await?;

        Ok(resp)
    }

    /// Gets a list of available files, that user have uploaded before
    pub async fn get_files(&mut self) -> anyhow::Result<Vec<GigaFile>> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
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
        let api_url = BASE_URL.to_owned() + "/v1/files";
        let mut files: HashMap<String, Vec<GigaFile>> =
            self.httpclient.get(&api_url, headers).await?;

        Ok(files.remove_entry("data").unwrap().1)
    }

    /// Sets to default if new_cfg is None, otherwise set to the passed config
    pub fn reset_msg_config(&mut self, new_cfg: Option<MessageConfig>) {
        self.message_cfg = new_cfg.unwrap_or_default();
    }
    pub fn get_current_config(&self) -> MessageConfig {
        self.message_cfg.clone()
    }

    /// Gets an OAuth config, needed for requests to the API
    async fn get_auth_token(&mut self) -> anyhow::Result<AccessToken> {
        let mut auth_token = self.auth_token.lock().unwrap();
        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            > auth_token.as_ref().map_or(0, |tok| tok.expires_at)
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

            *auth_token = Some(tok);
        }

        Ok(auth_token.clone().unwrap())
    }

    // Files
    /// Uploads a file to the GigaChat storage
    pub async fn upload_file(&mut self, filepath: PathBuf) -> anyhow::Result<GigaFile> {
        let file = tokio::fs::read(&filepath).await?;

        let mime_type = tree_magic::from_filepath(&filepath);
        let filename = filepath.file_name().unwrap().to_str().unwrap().to_owned();

        let form = Form::new()
            .part(
                "file",
                Part::bytes(file)
                    .file_name(filename)
                    .mime_str(&mime_type)
                    .unwrap(),
            )
            .text("purpose", "general");

        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.get_auth_token().await.unwrap().access_token
            ))
            .unwrap(),
        );

        let file: GigaFile = self
            .httpclient
            .post_multipart(&(BASE_URL.to_owned() + "/v1/files"), form, headers)
            .await?;

        Ok(file)
    }

    /// Deletes a file from the storage
    pub async fn delete_file(&mut self, file_id: &str) -> anyhow::Result<()> {
        let api_url = format!(
            "https://gigachat.devices.sberbank.ru/api/v1/files/{}/delete",
            file_id
        );

        let mut headers = HeaderMap::new();
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.get_auth_token().await.unwrap().access_token
            ))
            .unwrap(),
        );

        let json_obj: serde_json::Value = self
            .httpclient
            .post_data(&api_url, String::new(), headers)
            .await?;

        if !json_obj.get("deleted").unwrap().as_bool().unwrap_or(false) {
            return Err(anyhow!("File was not deleted"));
        }

        Ok(())
    }
}

pub struct ClientBuilder {
    msg_cfg: Option<MessageConfig>,
    basic_token: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            msg_cfg: None,
            basic_token: None,
        }
    }
    pub fn set_msg_cfg(mut self, msg_cfg: MessageConfig) -> Self {
        self.msg_cfg = msg_cfg.into();
        self
    }
    pub fn set_basic_token(mut self, basic_token: &str) -> Self {
        self.basic_token = basic_token.to_owned().into();
        self
    }
    pub fn build(self) -> GigaClient {
        GigaClient {
            basic_token: self.basic_token.expect("Token must be set"),
            auth_token: Arc::new(Mutex::new(None)),
            message_cfg: self.msg_cfg.unwrap_or_default(),
            uuid: uuid::Uuid::new_v4().to_string(),
            httpclient: HttpClient::new(),
        }
    }
}
