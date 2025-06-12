# Proxy Configuration

```rust
use binance_sdk::vip_loan;
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

let client = vip_loan::VIPLoanRestApi::production(configuration);
let params = vip_loan::rest_api::GetCollateralAssetDataParams::default();
let response = client.get_collateral_asset_data(params).await?;
```
