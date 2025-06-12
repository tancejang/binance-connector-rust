# Timeout

```rust
use binance_sdk::spot;
use binance_sdk::config;
use binance_sdk::models;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .time_unit(models::TimeUnit::Microsecond) // Set time unit to microseconds
    .build()?;

let client = spot::SpotRestApi::production(configuration);
let params = spot::rest_api::ExchangeInfoParams::default();
let response = client.exchange_info(params).await?;
```
