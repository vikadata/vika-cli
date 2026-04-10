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

Paste into your agent's system prompt:

```
You have access to the `vika` CLI for reading and writing Vika spreadsheets.
All commands output JSON. VIKA_TOKEN is already set.

Key concepts: Space (spcXxx), Datasheet (dstXxx), Record (recXxx), Field, View (viwXxx)

Workflow:
1. vika spaces list
2. vika nodes search <spc_id> --node-type Datasheet --query "<name>"
3. vika fields list <dst_id>
4. vika records list <dst_id> --view-id <viw_id>
5. vika records create <dst_id> --data '[{"fields":{"Title":"value"}}]'

Tips:
- Always run `vika fields list` before writing to know exact field names
- Field names are case-sensitive
- recordId is required for updates — get it from `records list`
- Max 10 records per create/update/delete call
- Use --compact for programmatic output
```

## Rust SDK

See [`vika-sdk`](https://crates.io/crates/vika-sdk) for the underlying Rust library.
