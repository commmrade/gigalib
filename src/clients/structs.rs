use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: u64,
}
