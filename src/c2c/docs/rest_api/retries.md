# Retries Configuration

```rust
use binance_sdk::c2c;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = c2c::C2CRestApi::production(configuration);
let params = c2c::rest_api::GetC2CTradeHistoryParams::default();
let response = client.get_c2_c_trade_history(params).await?;
```
