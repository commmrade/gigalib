use serde::Deserialize;

/// Struct used for storing information about a file
#[derive(Deserialize, Debug)]
pub struct GigaFile {
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub id: String,
    pub object: String,
    pub purpose: String,
    pub access_policy: String,
}
