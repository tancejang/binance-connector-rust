# Key Pair Based Authentication

```rust
use binance_sdk::derivatives_trading_portfolio_margin;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = derivatives_trading_portfolio_margin::DerivativesTradingPortfolioMarginRestApi::production(configuration);
let params = derivatives_trading_portfolio_margin::rest_api::AccountInformationParams::default();
let response = client.account_information(params).await?;
```
