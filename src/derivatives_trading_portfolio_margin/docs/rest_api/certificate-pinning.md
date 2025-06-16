# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::derivatives_trading_portfolio_margin;
use binance_sdk::config;

let pem = std::fs::read("/path/to/pinned_cert.pem");
let cert = reqwest::Certificate::from_pem(&pem);

let custom_agent = config::HttpAgent(std::sync::Arc::new(|builder: reqwest::ClientBuilder| {
    builder.add_root_certificate(cert.clone())
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = derivatives_trading_portfolio_margin::DerivativesTradingPortfolioMarginRestApi::production(configuration);
let params = derivatives_trading_portfolio_margin::rest_api::AccountInformationParams::default();
let response = client.account_information(params).await?;
```
