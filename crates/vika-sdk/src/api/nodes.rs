use crate::client::VIkaClient;
use crate::error::Result;
use crate::types::*;

pub struct NodesApi<'a> {
    client: &'a VIkaClient,
    space_id: &'a str,
}

impl<'a> NodesApi<'a> {
    pub fn new(client: &'a VIkaClient, space_id: &'a str) -> Self {
        Self { client, space_id }
    }

    pub async fn list(&self) -> Result<NodesData> {
        let path = format!("/spaces/{}/nodes", self.space_id);
        self.client.get_json(&path, &[]).await
    }

    pub async fn get(&self, node_id: &str) -> Result<Node> {
        let path = format!("/spaces/{}/nodes/{}", self.space_id, node_id);
        self.client.get_json(&path, &[]).await
    }

    pub async fn search(&self, node_type: &str, query: Option<&str>) -> Result<NodesData> {
        // v2 endpoint for search
        let url = format!(
            "{}/fusion/v2/spaces/{}/nodes",
            self.client.base_url, self.space_id
        );
        let mut params: Vec<(&str, &str)> = vec![("type", node_type)];
        if let Some(q) = query {
            params.push(("query", q));
        }
        let rb = self.client.auth(self.client.http.get(&url)).query(&params);
        let resp = rb.send().await?;
        let api: crate::types::ApiResponse<NodesData> = resp.json().await?;
        if !api.success {
            return Err(crate::error::VIkaError::Api { code: api.code, message: api.message });
        }
        api.data.ok_or_else(|| crate::error::VIkaError::Api { code: api.code, message: "empty data".into() })
    }
}
