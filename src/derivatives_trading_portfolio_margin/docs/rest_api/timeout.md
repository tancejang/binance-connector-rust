# Timeout

```rust
use binance_sdk::derivatives_trading_portfolio_margin;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = derivatives_trading_portfolio_margin::DerivativesTradingPortfolioMarginRestApi::production(configuration);
let params = derivatives_trading_portfolio_margin::rest_api::AccountInformationParams::default();
let response = client.account_information(params).await?;
```
