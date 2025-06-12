# Retries Configuration

```rust
use binance_sdk::convert;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = convert::ConvertRestApi::production(configuration);
let params = convert::rest_api::ListAllConvertPairsParams::default();
let response = client.list_all_convert_pairs(params).await?;
```
