use mockito::Server;
use vika_sdk::{VIkaClient, api::{records::RecordsApi, fields::FieldsApi, views::ViewsApi, spaces::SpacesApi, nodes::NodesApi}};

fn client(server: &Server) -> VIkaClient {
    VIkaClient::with_host("test_token", server.url())
}

// ── Records ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_records_list() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/datasheets/dst1/records")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"pageNum":1,"pageSize":100,"total":2,"records":[{"recordId":"rec1","fields":{"Title":"Hello"}},{"recordId":"rec2","fields":{"Title":"World"}}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = RecordsApi::new(&c, "dst1");
    let page = api.list(&[]).await.unwrap();
    assert_eq!(page.total, 2);
    assert_eq!(page.records[0].record_id, "rec1");
    assert_eq!(page.records[1].fields["Title"], "World");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_records_create() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/fusion/v1/datasheets/dst1/records")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"records":[{"recordId":"recNew","fields":{"Title":"New"}}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = RecordsApi::new(&c, "dst1");
    let mut fields = std::collections::HashMap::new();
    fields.insert("Title".to_string(), serde_json::json!("New"));
    let result = api.create(vec![vika_sdk::types::RecordFields { fields }], None).await.unwrap();
    assert_eq!(result.records[0].record_id, "recNew");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_records_update() {
    let mut server = Server::new_async().await;
    let mock = server.mock("PATCH", "/fusion/v1/datasheets/dst1/records")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"records":[{"recordId":"rec1","fields":{"Title":"Updated"}}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = RecordsApi::new(&c, "dst1");
    let mut fields = std::collections::HashMap::new();
    fields.insert("Title".to_string(), serde_json::json!("Updated"));
    let result = api.update(vec![vika_sdk::types::RecordUpdate { record_id: "rec1".into(), fields }], None).await.unwrap();
    assert_eq!(result.records[0].fields["Title"], "Updated");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_records_delete() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/fusion/v1/datasheets/dst1/records")
        .match_query(mockito::Matcher::UrlEncoded("recordIds".into(), "rec1,rec2".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":true}"#)
        .create_async().await;

    let c = client(&server);
    let api = RecordsApi::new(&c, "dst1");
    api.delete(&["rec1", "rec2"]).await.unwrap();
    mock.assert_async().await;
}

#[tokio::test]
async fn test_records_api_error() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/fusion/v1/datasheets/dst1/records")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":false,"code":401,"message":"身份认证失败","data":null}"#)
        .create_async().await;

    let c = client(&server);
    let api = RecordsApi::new(&c, "dst1");
    let err = api.list(&[]).await.unwrap_err();
    assert!(err.to_string().contains("401"));
}

// ── Fields ────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_fields_list() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/datasheets/dst1/fields")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"fields":[{"id":"fld1","name":"Title","type":"SingleText"}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = FieldsApi::new(&c, "spc1", "dst1");
    let data = api.list(None).await.unwrap();
    assert_eq!(data.fields.len(), 1);
    assert_eq!(data.fields[0].name, "Title");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_fields_create() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/fusion/v1/spaces/spc1/datasheets/dst1/fields")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"id":"fldNew","name":"Score"}}"#)
        .create_async().await;

    let c = client(&server);
    let api = FieldsApi::new(&c, "spc1", "dst1");
    let body = vika_sdk::types::CreateFieldBody { field_type: "Number".into(), name: "Score".into(), property: None };
    let data = api.create(body).await.unwrap();
    assert_eq!(data.id, "fldNew");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_fields_delete() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/fusion/v1/spaces/spc1/datasheets/dst1/fields/fld1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{}}"#)
        .create_async().await;

    let c = client(&server);
    let api = FieldsApi::new(&c, "spc1", "dst1");
    api.delete("fld1").await.unwrap();
    mock.assert_async().await;
}

// ── Views ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_views_list() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/datasheets/dst1/views")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"views":[{"id":"viw1","name":"Grid View","type":"Grid"}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = ViewsApi::new(&c, "dst1");
    let data = api.list().await.unwrap();
    assert_eq!(data.views[0].view_type, "Grid");
    mock.assert_async().await;
}

// ── Spaces ────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_spaces_list() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/spaces")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"spaces":[{"id":"spc1","name":"My Space","isAdmin":true}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = SpacesApi::new(&c);
    let data = api.list().await.unwrap();
    assert_eq!(data.spaces[0].id, "spc1");
    assert_eq!(data.spaces[0].is_admin, Some(true));
    mock.assert_async().await;
}

// ── Nodes ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_nodes_list() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/spaces/spc1/nodes")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"nodes":[{"id":"dst1","name":"Sheet","type":"Datasheet"}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = NodesApi::new(&c, "spc1");
    let data = api.list().await.unwrap();
    assert_eq!(data.nodes[0].node_type, "Datasheet");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_nodes_get() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v1/spaces/spc1/nodes/dst1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"id":"dst1","name":"Sheet","type":"Datasheet","isFav":false}}"#)
        .create_async().await;

    let c = client(&server);
    let api = NodesApi::new(&c, "spc1");
    let node = api.get("dst1").await.unwrap();
    assert_eq!(node.id, "dst1");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_nodes_search() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/fusion/v2/spaces/spc1/nodes")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success":true,"code":200,"message":"SUCCESS","data":{"nodes":[{"id":"dst1","name":"My Sheet","type":"Datasheet"}]}}"#)
        .create_async().await;

    let c = client(&server);
    let api = NodesApi::new(&c, "spc1");
    let data = api.search("Datasheet", Some("My Sheet")).await.unwrap();
    assert_eq!(data.nodes[0].name, "My Sheet");
    mock.assert_async().await;
}
