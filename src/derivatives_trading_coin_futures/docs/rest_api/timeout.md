# Timeout

```rust
use binance_sdk::derivatives_trading_coin_futures;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = derivatives_trading_coin_futures::DerivativesTradingCoinFuturesRestApi::production(configuration);
let response = client.exchange_information().await?;
```
