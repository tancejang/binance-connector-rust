# Time Unit Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;
use binance_sdk::models;

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .time_unit(models::TimeUnit::Microsecond) // Set time unit to microseconds
    .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;
```
