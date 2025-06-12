# Retries Configuration

```rust
use binance_sdk::mining;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = mining::MiningRestApi::production(configuration);
let response = client.acquiring_algorithm().await?;
```
