# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::rebate;
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

let client = rebate::RebateRestApi::production(configuration);
let params = rebate::rest_api::GetSpotRebateHistoryRecordsParams::default();
let response = client.get_spot_rebate_history_records(params).await?;
```
