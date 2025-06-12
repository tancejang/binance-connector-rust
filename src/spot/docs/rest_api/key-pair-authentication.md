# Key Pair Based Authentication

```rust
use binance_sdk::spot;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = spot::SpotRestApi::production(configuration);
let params = spot::rest_api::ExchangeInfoParams::default();
let response = client.exchange_info(params).await?;
```
