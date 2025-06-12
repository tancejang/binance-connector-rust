# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::simple_earn;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = simple_earn::SimpleEarnRestApi::production(configuration);
let params = simple_earn::rest_api::GetSimpleEarnFlexibleProductListParams::default();
let response = client.get_simple_earn_flexible_product_list(params).await?;
```
