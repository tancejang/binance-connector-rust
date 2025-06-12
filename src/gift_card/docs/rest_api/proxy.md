# Proxy Configuration

```rust
use binance_sdk::gift_card;
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

let client = gift_card::GiftCardRestApi::production(configuration);
let params = gift_card::rest_api::CreateASingleTokenGiftCardParams::builder("6H9EKF5ECCWFBHGE".to_string(), 1000.0).build()?;
let response = client.create_a_single_token_gift_card(params).await?;
```
