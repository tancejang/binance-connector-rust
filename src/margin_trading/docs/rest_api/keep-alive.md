# Keep-Alive Configuration

```rust
use binance_sdk::margin_trading;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .keep_alive(false) // default is true
    .build()?;

let client = margin_trading::MarginTradingRestApi::production(configuration);
let params = margin_trading::rest_api::GetSummaryOfMarginAccountParams::default();
let response = client.get_summary_of_margin_account(params).await?;
```
