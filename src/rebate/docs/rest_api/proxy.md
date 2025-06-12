# Proxy Configuration

```rust
use binance_sdk::rebate;
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

let client = rebate::RebateRestApi::production(configuration);
let params = rebate::rest_api::GetSpotRebateHistoryRecordsParams::default();
let response = client.get_spot_rebate_history_records(params).await?;
```
