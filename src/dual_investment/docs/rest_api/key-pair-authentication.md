# Key Pair Based Authentication

```rust
use binance_sdk::dual_investment;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = dual_investment::DualInvestmentRestApi::production(configuration);
let params = dual_investment::rest_api::GetDualInvestmentPositionsParams::default();
let response = client.get_dual_investment_positions(params).await?;
```
