use crate::client::VIkaClient;
use crate::error::Result;
use crate::types::*;

pub struct SpacesApi<'a> {
    client: &'a VIkaClient,
}

impl<'a> SpacesApi<'a> {
    pub fn new(client: &'a VIkaClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<SpacesData> {
        self.client.get_json("/spaces", &[]).await
    }
}
