# Retries Configuration

```rust
use binance_sdk::derivatives_trading_coin_futures;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = derivatives_trading_coin_futures::DerivativesTradingCoinFuturesRestApi::production(configuration);
let response = client.exchange_information().await?;
```
