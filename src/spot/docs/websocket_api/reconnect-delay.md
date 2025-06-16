# Reconnect Delay Configuration

```rust
use binance_sdk::spot;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .reconnect_delay(3000) // Set reconnect delay to 3 seconds
    .build()?;

let client = spot::SpotWsApi::production(configuration);
let connection = client.connect().await?;
let params = spot::websocket_api::ExchangeInfoParams::default();
let response = connection.exchange_info(params).await?;
```
