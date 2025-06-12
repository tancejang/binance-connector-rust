# Proxy Configuration

```rust
use binance_sdk::dual_investment;
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

let client = dual_investment::DualInvestmentRestApi::production(configuration);
let params = dual_investment::rest_api::GetDualInvestmentPositionsParams::default();
let response = client.get_dual_investment_positions(params).await?;
```
