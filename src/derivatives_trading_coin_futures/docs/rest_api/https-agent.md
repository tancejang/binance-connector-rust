# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::derivatives_trading_coin_futures;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = derivatives_trading_coin_futures::DerivativesTradingCoinFuturesRestApi::production(configuration);
let response = client.exchange_information().await?;
```
