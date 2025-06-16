# Compression Configuration

```rust
use binance_sdk::derivatives_trading_usds_futures;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi::production(configuration);
let response = client.exchange_information().await?;
```
