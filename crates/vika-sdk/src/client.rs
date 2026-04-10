use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::error::{Result, VIkaError};
use crate::types::ApiResponse;

const DEFAULT_HOST: &str = "https://vika.cn";
const FUSION_PREFIX: &str = "/fusion/v1";

#[derive(Clone, Debug)]
pub struct VIkaClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) token: String,
}

impl VIkaClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_host(token, DEFAULT_HOST)
    }

    pub fn with_host(token: impl Into<String>, host: impl Into<String>) -> Self {
        let http = Client::builder()
            .user_agent("vika-sdk-rust/0.1.0")
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            base_url: host.into(),
            token: token.into(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let token = std::env::var("VIKA_TOKEN").map_err(|_| VIkaError::MissingToken)?;
        Ok(Self::new(token))
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}{}", self.base_url, FUSION_PREFIX, path)
    }

    pub(crate) fn auth(&self, rb: RequestBuilder) -> RequestBuilder {
        rb.header("Authorization", format!("Bearer {}", self.token))
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, path: &str, params: &[(&str, &str)]) -> Result<T> {
        let url = self.url(path);
        let rb = self.auth(self.http.get(&url)).query(params);
        self.send(rb).await
    }

    pub(crate) async fn post_json<B: serde::Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.url(path);
        let rb = self.auth(self.http.post(&url)).json(body);
        self.send(rb).await
    }

    pub(crate) async fn patch_json<B: serde::Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.url(path);
        let rb = self.auth(self.http.patch(&url)).json(body);
        self.send(rb).await
    }

    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, path: &str, params: &[(&str, &str)]) -> Result<T> {
        let url = self.url(path);
        let rb = self.auth(self.http.delete(&url)).query(params);
        self.send(rb).await
    }

    pub(crate) async fn delete_no_body(&self, path: &str) -> Result<()> {
        let url = self.url(path);
        let rb = self.auth(self.http.delete(&url));
        let resp = rb.send().await?;
        let api: ApiResponse<serde_json::Value> = resp.json().await?;
        if !api.success {
            return Err(VIkaError::Api { code: api.code, message: api.message });
        }
        Ok(())
    }

    async fn send<T: DeserializeOwned>(&self, rb: RequestBuilder) -> Result<T> {
        let resp = rb.send().await?;
        let api: ApiResponse<T> = resp.json().await?;
        if !api.success {
            return Err(VIkaError::Api { code: api.code, message: api.message });
        }
        api.data.ok_or_else(|| VIkaError::Api { code: api.code, message: "empty data".into() })
    }
}
