# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::copy_trading;
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

let client = copy_trading::CopyTradingRestApi::production(configuration);
let params = copy_trading::rest_api::GetFuturesLeadTraderStatusParams::default();
let response = client.get_futures_lead_trader_status(params).await?;
```
