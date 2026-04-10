use crate::client::VIkaClient;
use crate::error::Result;
use crate::types::*;

pub struct RecordsApi<'a> {
    client: &'a VIkaClient,
    datasheet_id: &'a str,
}

impl<'a> RecordsApi<'a> {
    pub fn new(client: &'a VIkaClient, datasheet_id: &'a str) -> Self {
        Self { client, datasheet_id }
    }

    pub async fn list(&self, params: &[(&str, &str)]) -> Result<RecordsPage> {
        let path = format!("/datasheets/{}/records", self.datasheet_id);
        self.client.get_json(&path, params).await
    }

    pub async fn create(&self, records: Vec<RecordFields>, field_key: Option<String>) -> Result<CreateRecordsData> {
        let path = format!("/datasheets/{}/records", self.datasheet_id);
        let body = CreateRecordsBody { records, field_key };
        self.client.post_json(&path, &body).await
    }

    pub async fn update(&self, records: Vec<RecordUpdate>, field_key: Option<String>) -> Result<CreateRecordsData> {
        let path = format!("/datasheets/{}/records", self.datasheet_id);
        let body = UpdateRecordsBody { records, field_key };
        self.client.patch_json(&path, &body).await
    }

    pub async fn delete(&self, record_ids: &[&str]) -> Result<()> {
        let ids = record_ids.join(",");
        let path = format!("/datasheets/{}/records", self.datasheet_id);
        let params = [("recordIds", ids.as_str())];
        self.client.delete_json::<bool>(&path, &params).await?;
        Ok(())
    }
}
