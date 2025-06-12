# Key Pair Based Authentication

```rust
use binance_sdk::wallet;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = wallet::WalletRestApi::production(configuration);
let params = wallet::rest_api::AccountInfoParams::default();
let response = client.account_info(params).await?;
```
