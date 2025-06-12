# Timeout

```rust
use binance_sdk::simple_earn;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = simple_earn::SimpleEarnRestApi::production(configuration);
let params = simple_earn::rest_api::GetSimpleEarnFlexibleProductListParams::default();
let response = client.get_simple_earn_flexible_product_list(params).await?;
```
