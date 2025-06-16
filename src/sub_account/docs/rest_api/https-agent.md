# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::sub_account;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = sub_account::SubAccountRestApi::production(configuration);
let params = sub_account::rest_api::GetSummaryOfSubAccountsMarginAccountParams::default();
let response = client.get_summary_of_sub_accounts_margin_account(params).await?;
```
