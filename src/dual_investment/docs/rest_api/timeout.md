# Timeout

```rust
use binance_sdk::dual_investment;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = dual_investment::DualInvestmentRestApi::production(configuration);
let params = dual_investment::rest_api::GetDualInvestmentPositionsParams::default();
let response = client.get_dual_investment_positions(params).await?;
```
