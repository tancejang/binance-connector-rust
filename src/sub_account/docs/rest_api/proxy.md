# Proxy Configuration

```rust
use binance_sdk::sub_account;
use binance_sdk::config;

let proxy_config = config::ProxyConfig {
    host: "127.0.0.1".to_string(),
    port: 8080,
    protocol: Some("http".to_string()),
    auth: Some(config::ProxyAuth {
        username: "proxy-user".to_string(),
        password: "proxy-password".to_string(),
    }),
};

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .proxy(proxy_config)
    .build()?;

let client = sub_account::SubAccountRestApi::production(configuration);
let params = sub_account::rest_api::GetSummaryOfSubAccountsMarginAccountParams::default();
let response = client.get_summary_of_sub_accounts_margin_account(params).await?;
```
