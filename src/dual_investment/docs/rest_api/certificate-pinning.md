# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::dual_investment;
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

let client = dual_investment::DualInvestmentRestApi::production(configuration);
let params = dual_investment::rest_api::GetDualInvestmentPositionsParams::default();
let response = client.get_dual_investment_positions(params).await?;
```
