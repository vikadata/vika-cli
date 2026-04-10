pub mod api;
pub mod client;
pub mod error;
pub mod types;

pub use client::VIkaClient;
pub use error::{Result, VIkaError};

use api::{fields::FieldsApi, nodes::NodesApi, records::RecordsApi, spaces::SpacesApi, views::ViewsApi};

/// High-level entry point.
pub struct Vika {
    client: VIkaClient,
}

impl Vika {
    pub fn new(token: impl Into<String>) -> Self {
        Self { client: VIkaClient::new(token) }
    }

    pub fn with_host(token: impl Into<String>, host: impl Into<String>) -> Self {
        Self { client: VIkaClient::with_host(token, host) }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self { client: VIkaClient::from_env()? })
    }

    pub fn records<'a>(&'a self, datasheet_id: &'a str) -> RecordsApi<'a> {
        RecordsApi::new(&self.client, datasheet_id)
    }

    pub fn fields<'a>(&'a self, space_id: &'a str, datasheet_id: &'a str) -> FieldsApi<'a> {
        FieldsApi::new(&self.client, space_id, datasheet_id)
    }

    pub fn views<'a>(&'a self, datasheet_id: &'a str) -> ViewsApi<'a> {
        ViewsApi::new(&self.client, datasheet_id)
    }

    pub fn spaces(&self) -> SpacesApi<'_> {
        SpacesApi::new(&self.client)
    }

    pub fn nodes<'a>(&'a self, space_id: &'a str) -> NodesApi<'a> {
        NodesApi::new(&self.client, space_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_url() {
        let client = VIkaClient::new("test_token");
        assert_eq!(client.url("/spaces"), "https://vika.cn/fusion/v1/spaces");
    }

    #[test]
    fn test_from_env_missing() {
        std::env::remove_var("VIKA_TOKEN");
        let result = VIkaClient::from_env();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VIkaError::MissingToken));
    }

    #[test]
    fn test_from_env_present() {
        std::env::set_var("VIKA_TOKEN", "tok_test");
        let result = VIkaClient::from_env();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().token, "tok_test");
        std::env::remove_var("VIKA_TOKEN");
    }

    #[test]
    fn test_vika_new() {
        let vika = Vika::new("mytoken");
        assert_eq!(vika.client.token, "mytoken");
    }

    #[test]
    fn test_vika_with_host() {
        let vika = Vika::with_host("tok", "https://custom.host");
        assert_eq!(vika.client.base_url, "https://custom.host");
    }

    #[test]
    fn test_record_serialization() {
        use std::collections::HashMap;
        use crate::types::{RecordFields, CreateRecordsBody};

        let mut fields = HashMap::new();
        fields.insert("Title".to_string(), serde_json::json!("Hello"));
        let body = CreateRecordsBody {
            records: vec![RecordFields { fields }],
            field_key: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("Title"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_api_response_deserialize_success() {
        use crate::types::ApiResponse;
        let json = r#"{"success":true,"code":200,"message":"SUCCESS","data":{"spaces":[]}}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        assert_eq!(resp.code, 200);
    }

    #[test]
    fn test_api_response_deserialize_error() {
        use crate::types::ApiResponse;
        let json = r#"{"success":false,"code":401,"message":"身份认证失败","data":null}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(!resp.success);
        assert_eq!(resp.code, 401);
    }

    #[test]
    fn test_record_deserialize() {
        use crate::types::Record;
        let json = r#"{"recordId":"rec123","fields":{"Name":"Alice"},"createdAt":1000,"updatedAt":2000}"#;
        let record: Record = serde_json::from_str(json).unwrap();
        assert_eq!(record.record_id, "rec123");
        assert_eq!(record.fields["Name"], serde_json::json!("Alice"));
    }

    #[test]
    fn test_field_deserialize() {
        use crate::types::Field;
        let json = r#"{"id":"fld1","name":"Title","type":"SingleText","editable":true}"#;
        let field: Field = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, "fld1");
        assert_eq!(field.field_type, "SingleText");
    }

    #[test]
    fn test_view_deserialize() {
        use crate::types::View;
        let json = r#"{"id":"viw1","name":"Grid View","type":"Grid"}"#;
        let view: View = serde_json::from_str(json).unwrap();
        assert_eq!(view.view_type, "Grid");
    }

    #[test]
    fn test_space_deserialize() {
        use crate::types::Space;
        let json = r#"{"id":"spc1","name":"My Space","isAdmin":true}"#;
        let space: Space = serde_json::from_str(json).unwrap();
        assert_eq!(space.id, "spc1");
        assert_eq!(space.is_admin, Some(true));
    }

    #[test]
    fn test_node_deserialize() {
        use crate::types::Node;
        let json = r#"{"id":"dst1","name":"My Sheet","type":"Datasheet","isFav":false}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(node.node_type, "Datasheet");
    }

    #[test]
    fn test_update_records_body_serialization() {
        use std::collections::HashMap;
        use crate::types::{RecordUpdate, UpdateRecordsBody};

        let mut fields = HashMap::new();
        fields.insert("Status".to_string(), serde_json::json!("done"));
        let body = UpdateRecordsBody {
            records: vec![RecordUpdate { record_id: "rec1".into(), fields }],
            field_key: Some("name".into()),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("rec1"));
        assert!(json.contains("done"));
    }
}
