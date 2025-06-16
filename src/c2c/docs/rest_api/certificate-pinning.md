# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::c2c;
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

let client = c2c::C2CRestApi::production(configuration);
let params = c2c::rest_api::GetC2CTradeHistoryParams::default();
let response = client.get_c2_c_trade_history(params).await?;
```
