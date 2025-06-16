# Retries Configuration

```rust
use binance_sdk::pay;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = pay::PayRestApi::production(configuration);
let params = pay::rest_api::GetPayTradeHistoryParams::default();
let response = client.get_pay_trade_history(params).await?;
```
