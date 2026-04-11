# vikacli

CLI for the [Vika](https://vika.cn) (vikadata) API — designed for AI agents.

All commands output JSON. Binary name is `vika`.

## Install

**Download pre-built binary (recommended)**

Go to the [Releases page](https://github.com/vikadata/vika-cli/releases) and download for your platform:

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
```

**Or build from source**

```bash
cargo install vikacli
```

## Setup

```bash
export VIKA_TOKEN=uskYourTokenHere
```

## Usage

```bash
# Discover
vika spaces list
vika nodes list <space_id>
vika nodes search <space_id> --node-type Datasheet --query "my sheet"

# Read
vika records list <datasheet_id>
vika records list <datasheet_id> --view-id <view_id> --page-size 50
vika records list <datasheet_id> --filter '{Status}="Done"'
vika fields list <datasheet_id>
vika views list <datasheet_id>

# Write
vika records create <datasheet_id> --data '[{"fields":{"Title":"Hello"}}]'
vika records update <datasheet_id> --data '[{"recordId":"recXxx","fields":{"Title":"Updated"}}]'
vika records delete <datasheet_id> --ids recXxx,recYyy

# Compact JSON (for piping)
vika records list <datasheet_id> --compact | jq '.records[0]'
```

## For AI Agents

Add `vika` as a shell tool in your agent framework, then give the agent this context:

```
Tool: vika
Description: Read and write data in Vika (vikadata) spreadsheets. All output is JSON.
Usage: vika <command> [options]

Key concepts:
- Space (spcXxx): workspace containing nodes
- Datasheet (dstXxx): spreadsheet with records, fields, and views
- Record (recXxx): a row
- Field: a column (Title, Status, Date, etc.)
- View (viwXxx): filtered/sorted perspective on a datasheet

Available commands:
  vika spaces list
  vika nodes list <space_id>
  vika nodes search <space_id> --node-type Datasheet --query "<name>"
  vika nodes get <space_id> <node_id>
  vika records list <datasheet_id> [--view-id <id>] [--filter '<formula>'] [--page-size <n>]
  vika records create <datasheet_id> --data '[{"fields":{"Title":"value"}}]'
  vika records update <datasheet_id> --data '[{"recordId":"recXxx","fields":{"Title":"new"}}]'
  vika records delete <datasheet_id> --ids recXxx,recYyy
  vika fields list <datasheet_id>
  vika views list <datasheet_id>

Rules:
- Always run `vika fields list <dst_id>` before creating/updating records
- Field names are case-sensitive
- recordId is required for updates — get it from `records list`
- Max 10 records per create/update/delete call
- Use --compact for programmatic output
- Filter syntax: --filter '{FieldName}="value"'
```

## Rust SDK

See [`vika-sdk`](https://crates.io/crates/vika-sdk) for the underlying Rust library.
