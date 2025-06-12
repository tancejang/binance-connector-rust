# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::dual_investment;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = dual_investment::DualInvestmentRestApi::production(configuration);
let params = dual_investment::rest_api::GetDualInvestmentPositionsParams::default();
let response = client.get_dual_investment_positions(params).await?;
```
