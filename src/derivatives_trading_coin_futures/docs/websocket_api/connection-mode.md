# Connection Mode Configuration

```rust
use binance_sdk::derivatives_trading_coin_futures;
use binance_sdk::config;

let configuration = config::ConfigurationWebsocketApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .mode(models::WebsocketMode::Pool(3)) // Use pool mode with a pool size of 3
    .build()?;

let client = derivatives_trading_coin_futures::DerivativesTradingCoinFuturesWsApi::production(configuration);
let connection = client.connect().await?;
let params = derivatives_trading_coin_futures::websocket_api::AccountInformationParams::default();
let response = connection.account_information(params).await?;
```
