use serde::Deserialize;

/// GigaChat OAuth token, that is needed for making requests to the API
#[derive(Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: u64,
}
