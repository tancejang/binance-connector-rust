# Proxy Configuration

```rust
use binance_sdk::pay;
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

let client = pay::PayRestApi::production(configuration);
let params = pay::rest_api::GetPayTradeHistoryParams::default();
let response = client.get_pay_trade_history(params).await?;
```
