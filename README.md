# vika.rust

Rust monorepo for the [Vika](https://vika.cn) (vikadata) API.

| Crate | Description |
|-------|-------------|
| [`vika-sdk`](crates/vika-sdk) | Rust SDK library |
| [`vika-cli`](crates/vika-cli) | CLI tool — designed for AI agents |

---

## For Humans

### Install the CLI

**Option 1 — Download pre-built binary (recommended)**

Go to the [Releases page](https://github.com/vikadata/vika.rust/releases) and download the binary for your platform:

| Platform | File |
|----------|------|
| Linux x86_64 | `vika-linux-x86_64` |
| Linux ARM64 | `vika-linux-aarch64` |
| macOS x86_64 | `vika-macos-x86_64` |
| macOS Apple Silicon | `vika-macos-aarch64` |
| Windows x86_64 | `vika-windows-x86_64.exe` |

```bash
# Linux / macOS
chmod +x vika-linux-x86_64
sudo mv vika-linux-x86_64 /usr/local/bin/vika

# Verify
vika --version
```

**Option 2 — Build from source**

```bash
# Requires Rust: https://rustup.rs
cargo install --git https://github.com/vikadata/vika.rust vika-cli
```

### Get your API Token

1. Log in to [vika.cn](https://vika.cn)
2. Click your avatar (bottom-left) → **Personal Settings** → **API Token**
3. Copy the token

### Configure

```bash
export VIKA_TOKEN=uskYourTokenHere
```

Add to `~/.bashrc` or `~/.zshrc` to persist.

### Usage

```bash
# List all spaces
vika spaces list

# List top-level nodes in a space
vika nodes list spcXxxYourSpaceId

# Search for datasheets by name
vika nodes search spcXxx --node-type Datasheet --query "my sheet"

# Get node details
vika nodes get spcXxx dstXxx

# List records (first 100)
vika records list dstXxx

# List records with filters
vika records list dstXxx --view-id viwXxx --page-size 50 --filter '{Title}="Hello"'

# Create records
vika records create dstXxx --data '[{"fields":{"Title":"Hello","Status":"Todo"}}]'

# Update records
vika records update dstXxx --data '[{"recordId":"recXxx","fields":{"Status":"Done"}}]'

# Delete records
vika records delete dstXxx --ids recXxx,recYyy

# List fields
vika fields list dstXxx

# List views
vika views list dstXxx

# Compact JSON output (useful for piping)
vika records list dstXxx --compact | jq '.records[0]'
```

### Use the Rust SDK

```toml
# Cargo.toml
[dependencies]
vika-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use vika_sdk::Vika;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads VIKA_TOKEN from environment
    let vika = Vika::from_env()?;

    // List spaces
    let spaces = vika.spaces().list().await?;
    println!("Spaces: {:?}", spaces.spaces);

    // List records
    let page = vika.records("dstXxx").list(&[]).await?;
    println!("{} total records", page.total);
    for record in &page.records {
        println!("{}: {:?}", record.record_id, record.fields);
    }

    // Create records
    use std::collections::HashMap;
    use vika_sdk::types::RecordFields;
    let mut fields = HashMap::new();
    fields.insert("Title".to_string(), serde_json::json!("New row"));
    let created = vika.records("dstXxx")
        .create(vec![RecordFields { fields }], None)
        .await?;
    println!("Created: {}", created.records[0].record_id);

    Ok(())
}
```

---

## For AI Agents

### Quick Setup

Set the environment variable before running the agent:

```bash
export VIKA_TOKEN=uskYourTokenHere
```

### Tool / System Prompt Snippet

Paste this into your agent's system prompt or tool description:

```
You have access to the `vika` CLI for reading and writing data in Vika (vikadata) spreadsheets.
All commands output JSON. The VIKA_TOKEN environment variable is already set.

## Key concepts
- Space (spcXxx): a workspace containing nodes
- Datasheet (dstXxx): a spreadsheet with records, fields, and views
- Record (recXxx): a row in a datasheet
- Field: a column (Title, Status, Date, etc.)
- View (viwXxx): a filtered/sorted perspective on a datasheet

## Available commands

### Discover data
vika spaces list
vika nodes list <space_id>
vika nodes search <space_id> --node-type Datasheet --query "<name>"
vika nodes get <space_id> <node_id>

### Read
vika records list <datasheet_id>
vika records list <datasheet_id> --view-id <view_id>
vika records list <datasheet_id> --filter '{FieldName}="value"'
vika records list <datasheet_id> --fields "Title,Status" --page-size 100
vika fields list <datasheet_id>
vika views list <datasheet_id>

### Write
vika records create <datasheet_id> --data '[{"fields":{"Title":"value"}}]'
vika records update <datasheet_id> --data '[{"recordId":"recXxx","fields":{"Title":"new"}}]'
vika records delete <datasheet_id> --ids recXxx,recYyy

### Output
- Default: pretty-printed JSON
- Add --compact for minified JSON

## Workflow example
1. Find the right datasheet:
   vika spaces list
   vika nodes search spcXxx --node-type Datasheet --query "tasks"

2. Understand its structure:
   vika fields list dstXxx
   vika views list dstXxx

3. Read data:
   vika records list dstXxx --view-id viwXxx

4. Write data:
   vika records create dstXxx --data '[{"fields":{"Title":"New task","Status":"Todo"}}]'
```

### MCP / Tool Call Schema (JSON)

If your agent framework uses structured tool definitions:

```json
{
  "name": "vika",
  "description": "Read and write data in Vika spreadsheets. All output is JSON.",
  "parameters": {
    "type": "object",
    "properties": {
      "command": {
        "type": "string",
        "description": "Full vika CLI command, e.g. 'records list dstXxx' or 'records create dstXxx --data [{\"fields\":{\"Title\":\"Hello\"}}]'"
      }
    },
    "required": ["command"]
  }
}
```

Invoke as:
```bash
vika <command>
```

### Tips for Agents

- Always run `vika fields list <dst_id>` before creating/updating records to know exact field names
- Field names are **case-sensitive**
- `recordId` (e.g. `recXxx`) is required for updates — get it from `records list`
- Max 10 records per create/update/delete call — batch if needed
- Use `--compact` when parsing output programmatically to avoid whitespace issues
- Use `--filter` with Vika formula syntax, e.g. `--filter '{Status}="Done"'`

---

## Development

```bash
git clone https://github.com/vikadata/vika.rust
cd vika.rust
cargo test --workspace   # runs 37 tests (unit + mock integration, no API key needed)
cargo build --workspace
```

## Release

Push a tag to trigger CI:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will:
1. Publish `vika-sdk` to [crates.io](https://crates.io)
2. Build binaries for all platforms
3. Create a GitHub Release with all binaries attached

Requires `CARGO_REGISTRY_TOKEN` secret in repository settings.
