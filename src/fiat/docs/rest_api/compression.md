# Compression Configuration

```rust
use binance_sdk::fiat;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = fiat::FiatRestApi::production(configuration);
let params = fiat::rest_api::GetFiatDepositWithdrawHistoryParams::builder("0".to_string()).build()?;
let response = client.get_fiat_deposit_withdraw_history(params).await?;
```
