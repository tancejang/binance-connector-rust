# Key Pair Based Authentication

```rust
use binance_sdk::convert;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = convert::ConvertRestApi::production(configuration);
let params = convert::rest_api::ListAllConvertPairsParams::default();
let response = client.list_all_convert_pairs(params).await?;
```
