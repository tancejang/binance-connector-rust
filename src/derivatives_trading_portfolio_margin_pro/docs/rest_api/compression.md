# Compression Configuration

```rust
use binance_sdk::derivatives_trading_portfolio_margin_pro;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = derivatives_trading_portfolio_margin_pro::DerivativesTradingPortfolioMarginProRestApi::production(configuration);
let params = derivatives_trading_portfolio_margin_pro::rest_api::GetPortfolioMarginProAccountInfoParams::default();
let response = client.get_portfolio_margin_pro_account_info(params).await?;
```
