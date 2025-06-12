# Retries Configuration

```rust
use binance_sdk::copy_trading;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = copy_trading::CopyTradingRestApi::production(configuration);
let params = copy_trading::rest_api::GetFuturesLeadTraderStatusParams::default();
let response = client.get_futures_lead_trader_status(params).await?;
```
