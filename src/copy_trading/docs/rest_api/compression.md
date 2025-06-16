# Compression Configuration

```rust
use binance_sdk::copy_trading;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = copy_trading::CopyTradingRestApi::production(configuration);
let params = copy_trading::rest_api::GetFuturesLeadTraderStatusParams::default();
let response = client.get_futures_lead_trader_status(params).await?;
```
