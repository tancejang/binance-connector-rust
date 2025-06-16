# Key Pair Based Authentication

```rust
use binance_sdk::margin_trading;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = margin_trading::MarginTradingRestApi::production(configuration);
let params = margin_trading::rest_api::GetSummaryOfMarginAccountParams::default();
let response = client.get_summary_of_margin_account(params).await?;
```
