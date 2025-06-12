# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::gift_card;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = gift_card::GiftCardRestApi::production(configuration);
let params = gift_card::rest_api::CreateASingleTokenGiftCardParams::builder("6H9EKF5ECCWFBHGE".to_string(), 1000.0).build()?;
let response = client.create_a_single_token_gift_card(params).await?;
```
