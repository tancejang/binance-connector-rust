# Private Key Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;
```
