# Timeout Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(10000) // Set timeout to 10 seconds
    .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;
```
