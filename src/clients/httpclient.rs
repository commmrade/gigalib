use std::collections::HashMap;

use anyhow::anyhow;
use serde::Deserialize;

pub struct HttpClient {
    httpclient: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            httpclient: reqwest::ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("reqwest Client could not be created"),
        }
    }

    pub async fn post_form<R>(
        &self,
        api: &str,
        body: HashMap<String, String>,
        headers: reqwest::header::HeaderMap,
    ) -> anyhow::Result<R>
    where
        R: for<'a> Deserialize<'a>,
    {
        let resp = self
            .httpclient
            .post(api)
            .headers(headers)
            .form(&body)
            .send()
            .await
            .map_err(|why| anyhow!(format!("could not send request {}", why)))?;
        if !resp.status().is_success() {
            return Err(anyhow!(format!("Request is not successful: {}", resp.status().to_string())));
        }
        
        let resp_str: String = resp.text().await?;
        let r: R = serde_json::from_str(&resp_str)
            .map_err(|why| anyhow!(format!("Could not deserialize: {}", why)))?;
        Ok(r)
    }

    pub async fn post_data<S, R>(
        &self,
        api: &str,
        body: S,
        headers: reqwest::header::HeaderMap,
    ) -> anyhow::Result<R>
    where
        R: for<'a> Deserialize<'a>,
        reqwest::Body: From<S>,
    {
        let resp = self
            .httpclient
            .post(api)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|why| anyhow!(format!("Sending failure: {}", why)))?;

        if !resp.status().is_success() {
            return Err(anyhow!(format!("Request is not successful: {}", resp.status().to_string())));
        }

        let resp_str = resp.text().await?;
        let r: R = serde_json::from_str(&resp_str)
            .map_err(|why| anyhow!(format!("Could not deserialize {}", why)))?;

        Ok(r)
    }

    pub async fn get<R>(&self, api: &str, headers: reqwest::header::HeaderMap) -> anyhow::Result<R>
    where
        R: for<'a> Deserialize<'a>,
    {
        let resp = self
            .httpclient
            .get(api)
            .headers(headers)
            .send()
            .await
            .map_err(|why| anyhow!(format!("Sending failure: {}", why)))?;

        if !resp.status().is_success() {
            return Err(anyhow!(format!("Request is not successful: {}", resp.status().to_string())));
        }

        let resp_str: String = resp.text().await?;
        let r: R = serde_json::from_str(&resp_str)
            .map_err(|why| anyhow!(format!("Could not deserialize {}", why)))?;
        Ok(r)
    }
}
