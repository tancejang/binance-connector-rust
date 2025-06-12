# Key Pair Based Authentication

```rust
use binance_sdk::simple_earn;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = simple_earn::SimpleEarnRestApi::production(configuration);
let params = simple_earn::rest_api::GetSimpleEarnFlexibleProductListParams::default();
let response = client.get_simple_earn_flexible_product_list(params).await?;
```
