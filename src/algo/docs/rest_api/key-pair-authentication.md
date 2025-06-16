# Key Pair Based Authentication

```rust
use binance_sdk::algo;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = algo::AlgoRestApi::production(configuration);
let params = algo::rest_api::QueryHistoricalAlgoOrdersSpotAlgoParams::default();
let response = client.query_historical_algo_orders_spot_algo(params).await?;
```
