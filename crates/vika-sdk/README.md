# vika-sdk

Rust SDK for the [Vika](https://vika.cn) API.

```toml
[dependencies]
vika-sdk = "0.1"
```

```rust
use vika_sdk::Vika;

#[tokio::main]
async fn main() {
    let vika = Vika::from_env().unwrap(); // reads VIKA_TOKEN
    let page = vika.records("dstXxx").list(&[]).await.unwrap();
    println!("{} records", page.total);
}
```
