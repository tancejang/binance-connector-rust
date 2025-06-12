# Compression Configuration

```rust
use binance_sdk::derivatives_trading_options;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = derivatives_trading_options::DerivativesTradingOptionsRestApi::production(configuration);
let params = derivatives_trading_options::rest_api::OptionAccountInformationParams::default();
let response = client.option_account_information(params).await?;
```
