# Proxy Configuration

```rust
use binance_sdk::simple_earn;
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

let client = simple_earn::SimpleEarnRestApi::production(configuration);
let params = simple_earn::rest_api::GetSimpleEarnFlexibleProductListParams::default();
let response = client.get_simple_earn_flexible_product_list(params).await?;
```
