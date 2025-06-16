# Compression Configuration

```rust
use binance_sdk::algo;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = algo::AlgoRestApi::production(configuration);
let params = algo::rest_api::QueryHistoricalAlgoOrdersSpotAlgoParams::default();
let response = client.query_historical_algo_orders_spot_algo(params).await?;
```
