# Key Pair Based Authentication

```rust
use binance_sdk::copy_trading;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = copy_trading::CopyTradingRestApi::production(configuration);
let params = copy_trading::rest_api::GetFuturesLeadTraderStatusParams::default();
let response = client.get_futures_lead_trader_status(params).await?;
```
