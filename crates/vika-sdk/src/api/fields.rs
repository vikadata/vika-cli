use crate::client::VIkaClient;
use crate::error::Result;
use crate::types::*;

pub struct FieldsApi<'a> {
    client: &'a VIkaClient,
    space_id: &'a str,
    datasheet_id: &'a str,
}

impl<'a> FieldsApi<'a> {
    pub fn new(client: &'a VIkaClient, space_id: &'a str, datasheet_id: &'a str) -> Self {
        Self { client, space_id, datasheet_id }
    }

    pub async fn list(&self, view_id: Option<&str>) -> Result<FieldsData> {
        let path = format!("/datasheets/{}/fields", self.datasheet_id);
        let params: Vec<(&str, &str)> = view_id.map(|v| vec![("viewId", v)]).unwrap_or_default();
        self.client.get_json(&path, &params).await
    }

    pub async fn create(&self, body: CreateFieldBody) -> Result<CreateFieldData> {
        let path = format!("/spaces/{}/datasheets/{}/fields", self.space_id, self.datasheet_id);
        self.client.post_json(&path, &body).await
    }

    pub async fn delete(&self, field_id: &str) -> Result<()> {
        let path = format!("/spaces/{}/datasheets/{}/fields/{}", self.space_id, self.datasheet_id, field_id);
        self.client.delete_no_body(&path).await
    }
}
