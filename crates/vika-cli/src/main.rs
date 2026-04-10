use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use vika_sdk::types::{CreateFieldBody, RecordFields, RecordUpdate};
use vika_sdk::VIkaClient;

#[derive(Parser)]
#[command(name = "vika", about = "Vika API CLI for AI agents", version)]
struct Cli {
    /// API token (defaults to VIKA_TOKEN env var)
    #[arg(long, env = "VIKA_TOKEN", global = true)]
    token: Option<String>,

    /// API host (defaults to https://vika.cn)
    #[arg(long, env = "VIKA_HOST", global = true, default_value = "https://vika.cn")]
    host: String,

    /// Output compact JSON instead of pretty-printed
    #[arg(long, global = true)]
    compact: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Record operations
    Records {
        #[command(subcommand)]
        cmd: RecordsCmd,
    },
    /// Field operations
    Fields {
        #[command(subcommand)]
        cmd: FieldsCmd,
    },
    /// View operations
    Views {
        #[command(subcommand)]
        cmd: ViewsCmd,
    },
    /// Space operations
    Spaces {
        #[command(subcommand)]
        cmd: SpacesCmd,
    },
    /// Node operations
    Nodes {
        #[command(subcommand)]
        cmd: NodesCmd,
    },
}

// ── Records ───────────────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum RecordsCmd {
    /// List records in a datasheet
    List {
        datasheet_id: String,
        #[arg(long)] view_id: Option<String>,
        #[arg(long)] page_size: Option<u32>,
        #[arg(long)] page_num: Option<u32>,
        #[arg(long)] filter: Option<String>,
        #[arg(long)] fields: Option<String>,
        #[arg(long)] field_key: Option<String>,
    },
    /// Create records (JSON array of field objects)
    Create {
        datasheet_id: String,
        /// JSON: '[{"fields":{"Title":"value"}}]'
        #[arg(long)] data: String,
        #[arg(long)] field_key: Option<String>,
    },
    /// Update records (JSON array with recordId + fields)
    Update {
        datasheet_id: String,
        /// JSON: '[{"recordId":"recXxx","fields":{"Title":"new"}}]'
        #[arg(long)] data: String,
        #[arg(long)] field_key: Option<String>,
    },
    /// Delete records by IDs (comma-separated)
    Delete {
        datasheet_id: String,
        /// Comma-separated record IDs
        #[arg(long)] ids: String,
    },
}

// ── Fields ────────────────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum FieldsCmd {
    /// List fields in a datasheet
    List {
        datasheet_id: String,
        #[arg(long)] space_id: Option<String>,
        #[arg(long)] view_id: Option<String>,
    },
    /// Create a field
    Create {
        #[arg(long)] space_id: String,
        #[arg(long)] datasheet_id: String,
        #[arg(long, name = "type")] field_type: String,
        #[arg(long)] name: String,
        #[arg(long)] property: Option<String>,
    },
    /// Delete a field
    Delete {
        #[arg(long)] space_id: String,
        #[arg(long)] datasheet_id: String,
        #[arg(long)] field_id: String,
    },
}

// ── Views ─────────────────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum ViewsCmd {
    /// List views in a datasheet
    List { datasheet_id: String },
}

// ── Spaces ────────────────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum SpacesCmd {
    /// List all spaces
    List,
}

// ── Nodes ─────────────────────────────────────────────────────────────────────

#[derive(Subcommand)]
enum NodesCmd {
    /// List top-level nodes in a space
    List { space_id: String },
    /// Get node details
    Get { space_id: String, node_id: String },
    /// Search nodes by type and optional query
    Search {
        space_id: String,
        #[arg(long, name = "type")] node_type: String,
        #[arg(long)] query: Option<String>,
    },
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> Cli {
        Cli::parse_from(std::iter::once("vika").chain(args.iter().copied()))
    }

    #[test]
    fn test_spaces_list() {
        let cli = parse(&["--token", "tok", "spaces", "list"]);
        assert!(matches!(cli.command, Commands::Spaces { cmd: SpacesCmd::List }));
    }

    #[test]
    fn test_records_list_defaults() {
        let cli = parse(&["--token", "tok", "records", "list", "dstXxx"]);
        match cli.command {
            Commands::Records { cmd: RecordsCmd::List { datasheet_id, view_id, page_size, .. } } => {
                assert_eq!(datasheet_id, "dstXxx");
                assert!(view_id.is_none());
                assert!(page_size.is_none());
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_records_list_with_options() {
        let cli = parse(&["--token", "tok", "records", "list", "dstXxx",
            "--view-id", "viwAbc", "--page-size", "50"]);
        match cli.command {
            Commands::Records { cmd: RecordsCmd::List { view_id, page_size, .. } } => {
                assert_eq!(view_id.as_deref(), Some("viwAbc"));
                assert_eq!(page_size, Some(50));
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_records_create_parses_data() {
        let data = r#"[{"fields":{"Title":"Hello"}}]"#;
        let cli = parse(&["--token", "tok", "records", "create", "dstXxx", "--data", data]);
        match cli.command {
            Commands::Records { cmd: RecordsCmd::Create { datasheet_id, data: d, .. } } => {
                assert_eq!(datasheet_id, "dstXxx");
                let parsed: Vec<RecordFields> = serde_json::from_str(&d).unwrap();
                assert_eq!(parsed.len(), 1);
                assert_eq!(parsed[0].fields["Title"], serde_json::json!("Hello"));
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_records_delete_ids() {
        let cli = parse(&["--token", "tok", "records", "delete", "dstXxx", "--ids", "rec1,rec2"]);
        match cli.command {
            Commands::Records { cmd: RecordsCmd::Delete { ids, .. } } => {
                let id_list: Vec<&str> = ids.split(',').collect();
                assert_eq!(id_list, vec!["rec1", "rec2"]);
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_nodes_search() {
        let cli = parse(&["--token", "tok", "nodes", "search", "spcXxx",
            "--node-type", "Datasheet", "--query", "my sheet"]);
        match cli.command {
            Commands::Nodes { cmd: NodesCmd::Search { space_id, node_type, query } } => {
                assert_eq!(space_id, "spcXxx");
                assert_eq!(node_type, "Datasheet");
                assert_eq!(query.as_deref(), Some("my sheet"));
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_compact_flag() {
        let cli = parse(&["--token", "tok", "--compact", "spaces", "list"]);
        assert!(cli.compact);
    }

    #[test]
    fn test_host_override() {
        let cli = parse(&["--token", "tok", "--host", "https://custom.vika.cn", "spaces", "list"]);
        assert_eq!(cli.host, "https://custom.vika.cn");
    }

    #[test]
    fn test_fields_create() {
        let cli = parse(&["--token", "tok", "fields", "create",
            "--space-id", "spcXxx", "--datasheet-id", "dstXxx",
            "--field-type", "SingleText", "--name", "My Field"]);
        match cli.command {
            Commands::Fields { cmd: FieldsCmd::Create { space_id, datasheet_id, field_type, name, .. } } => {
                assert_eq!(space_id, "spcXxx");
                assert_eq!(datasheet_id, "dstXxx");
                assert_eq!(field_type, "SingleText");
                assert_eq!(name, "My Field");
            }
            _ => panic!("wrong command"),
        }
    }

    #[test]
    fn test_views_list() {
        let cli = parse(&["--token", "tok", "views", "list", "dstXxx"]);
        match cli.command {
            Commands::Views { cmd: ViewsCmd::List { datasheet_id } } => {
                assert_eq!(datasheet_id, "dstXxx");
            }
            _ => panic!("wrong command"),
        }
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let token = cli.token.context("API token required: set VIKA_TOKEN or pass --token")?;
    let client = VIkaClient::with_host(token, cli.host);

    let value = dispatch(&client, cli.command).await?;

    if cli.compact {
        println!("{}", serde_json::to_string(&value)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&value)?);
    }

    Ok(())
}

async fn dispatch(client: &VIkaClient, cmd: Commands) -> Result<serde_json::Value> {
    use vika_sdk::api::{fields::FieldsApi, nodes::NodesApi, records::RecordsApi, spaces::SpacesApi, views::ViewsApi};

    match cmd {
        Commands::Records { cmd } => match cmd {
            RecordsCmd::List { datasheet_id, view_id, page_size, page_num, filter, fields, field_key } => {
                let api = RecordsApi::new(client, &datasheet_id);
                let mut params: Vec<(&str, String)> = vec![];
                if let Some(v) = &view_id { params.push(("viewId", v.clone())); }
                if let Some(v) = page_size { params.push(("pageSize", v.to_string())); }
                if let Some(v) = page_num { params.push(("pageNum", v.to_string())); }
                if let Some(v) = &filter { params.push(("filterByFormula", v.clone())); }
                if let Some(v) = &fields { params.push(("fields", v.clone())); }
                if let Some(v) = &field_key { params.push(("fieldKey", v.clone())); }
                let p: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
                let data = api.list(&p).await?;
                Ok(serde_json::to_value(data)?)
            }
            RecordsCmd::Create { datasheet_id, data, field_key } => {
                let api = RecordsApi::new(client, &datasheet_id);
                let records: Vec<RecordFields> = serde_json::from_str(&data)
                    .context("--data must be a JSON array of {\"fields\":{...}}")?;
                let result = api.create(records, field_key).await?;
                Ok(serde_json::to_value(result)?)
            }
            RecordsCmd::Update { datasheet_id, data, field_key } => {
                let api = RecordsApi::new(client, &datasheet_id);
                let records: Vec<RecordUpdate> = serde_json::from_str(&data)
                    .context("--data must be a JSON array of {\"recordId\":\"...\",\"fields\":{...}}")?;
                let result = api.update(records, field_key).await?;
                Ok(serde_json::to_value(result)?)
            }
            RecordsCmd::Delete { datasheet_id, ids } => {
                let api = RecordsApi::new(client, &datasheet_id);
                let id_list: Vec<&str> = ids.split(',').map(str::trim).collect();
                api.delete(&id_list).await?;
                Ok(serde_json::json!({"deleted": id_list}))
            }
        },

        Commands::Fields { cmd } => match cmd {
            FieldsCmd::List { datasheet_id, space_id, view_id } => {
                let sid = space_id.as_deref().unwrap_or("");
                let api = FieldsApi::new(client, sid, &datasheet_id);
                let data = api.list(view_id.as_deref()).await?;
                Ok(serde_json::to_value(data)?)
            }
            FieldsCmd::Create { space_id, datasheet_id, field_type, name, property } => {
                let api = FieldsApi::new(client, &space_id, &datasheet_id);
                let prop = property.as_deref().map(serde_json::from_str).transpose()
                    .context("--property must be valid JSON")?;
                let body = CreateFieldBody { field_type, name, property: prop };
                let data = api.create(body).await?;
                Ok(serde_json::to_value(data)?)
            }
            FieldsCmd::Delete { space_id, datasheet_id, field_id } => {
                let api = FieldsApi::new(client, &space_id, &datasheet_id);
                api.delete(&field_id).await?;
                Ok(serde_json::json!({"deleted": field_id}))
            }
        },

        Commands::Views { cmd } => match cmd {
            ViewsCmd::List { datasheet_id } => {
                let api = ViewsApi::new(client, &datasheet_id);
                let data = api.list().await?;
                Ok(serde_json::to_value(data)?)
            }
        },

        Commands::Spaces { cmd } => match cmd {
            SpacesCmd::List => {
                let api = SpacesApi::new(client);
                let data = api.list().await?;
                Ok(serde_json::to_value(data)?)
            }
        },

        Commands::Nodes { cmd } => match cmd {
            NodesCmd::List { space_id } => {
                let api = NodesApi::new(client, &space_id);
                let data = api.list().await?;
                Ok(serde_json::to_value(data)?)
            }
            NodesCmd::Get { space_id, node_id } => {
                let api = NodesApi::new(client, &space_id);
                let data = api.get(&node_id).await?;
                Ok(serde_json::to_value(data)?)
            }
            NodesCmd::Search { space_id, node_type, query } => {
                let api = NodesApi::new(client, &space_id);
                let data = api.search(&node_type, query.as_deref()).await?;
                Ok(serde_json::to_value(data)?)
            }
        },
    }
}


