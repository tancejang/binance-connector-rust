# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::copy_trading;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = copy_trading::CopyTradingRestApi::production(configuration);
let params = copy_trading::rest_api::GetFuturesLeadTraderStatusParams::default();
let response = client.get_futures_lead_trader_status(params).await?;
```
