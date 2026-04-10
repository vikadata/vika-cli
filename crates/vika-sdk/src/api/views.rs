use crate::client::VIkaClient;
use crate::error::Result;
use crate::types::*;

pub struct ViewsApi<'a> {
    client: &'a VIkaClient,
    datasheet_id: &'a str,
}

impl<'a> ViewsApi<'a> {
    pub fn new(client: &'a VIkaClient, datasheet_id: &'a str) -> Self {
        Self { client, datasheet_id }
    }

    pub async fn list(&self) -> Result<ViewsData> {
        let path = format!("/datasheets/{}/views", self.datasheet_id);
        self.client.get_json(&path, &[]).await
    }
}
