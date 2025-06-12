# Retries Configuration

```rust
use binance_sdk::wallet;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = wallet::WalletRestApi::production(configuration);
let params = wallet::rest_api::AccountInfoParams::default();
let response = client.account_info(params).await?;
```
