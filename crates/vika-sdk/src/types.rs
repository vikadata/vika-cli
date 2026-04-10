use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}

// ── Records ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub record_id: String,
    pub fields: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordsPage {
    pub page_num: u32,
    pub page_size: u32,
    pub total: u32,
    pub records: Vec<Record>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecordsBody {
    pub records: Vec<RecordFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordFields {
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecordsBody {
    pub records: Vec<RecordUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordUpdate {
    pub record_id: String,
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRecordsData {
    pub records: Vec<Record>,
}

// ── Fields ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldsData {
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFieldBody {
    #[serde(rename = "type")]
    pub field_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFieldData {
    pub id: String,
    pub name: String,
}

// ── Views ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct View {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub view_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewsData {
    pub views: Vec<View>,
}

// ── Spaces ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpacesData {
    pub spaces: Vec<Space>,
}

// ── Nodes ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fav: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodesData {
    pub nodes: Vec<Node>,
}
