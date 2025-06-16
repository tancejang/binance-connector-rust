# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::algo;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = algo::AlgoRestApi::production(configuration);
let params = algo::rest_api::QueryHistoricalAlgoOrdersSpotAlgoParams::default();
let response = client.query_historical_algo_orders_spot_algo(params).await?;
```
