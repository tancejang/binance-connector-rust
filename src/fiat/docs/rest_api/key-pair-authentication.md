# Key Pair Based Authentication

```rust
use binance_sdk::fiat;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = fiat::FiatRestApi::production(configuration);
let params = fiat::rest_api::GetFiatDepositWithdrawHistoryParams::builder("0".to_string()).build()?;
let response = client.get_fiat_deposit_withdraw_history(params).await?;
```
