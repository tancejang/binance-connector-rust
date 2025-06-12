# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::derivatives_trading_portfolio_margin_pro;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = derivatives_trading_portfolio_margin_pro::DerivativesTradingPortfolioMarginProRestApi::production(configuration);
let params = derivatives_trading_portfolio_margin_pro::rest_api::GetPortfolioMarginProAccountInfoParams::default();
let response = client.get_portfolio_margin_pro_account_info(params).await?;
```
