# Key Pair Based Authentication

```rust
use binance_sdk::pay;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = pay::PayRestApi::production(configuration);
let params = pay::rest_api::GetPayTradeHistoryParams::default();
let response = client.get_pay_trade_history(params).await?;
```
