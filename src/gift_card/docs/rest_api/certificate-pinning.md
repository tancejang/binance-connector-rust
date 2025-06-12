# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::gift_card;
use binance_sdk::config;

let pem = std::fs::read("/path/to/pinned_cert.pem");
let cert = reqwest::Certificate::from_pem(&pem);

let custom_agent = config::HttpAgent(std::sync::Arc::new(|builder: reqwest::ClientBuilder| {
    builder.add_root_certificate(cert.clone())
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
