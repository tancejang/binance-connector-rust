# Compression Configuration

```rust
use binance_sdk::mining;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = mining::MiningRestApi::production(configuration);
let response = client.acquiring_algorithm().await?;
```
