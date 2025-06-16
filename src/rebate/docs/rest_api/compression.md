# Compression Configuration

```rust
use binance_sdk::rebate;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = rebate::RebateRestApi::production(configuration);
let params = rebate::rest_api::GetSpotRebateHistoryRecordsParams::default();
let response = client.get_spot_rebate_history_records(params).await?;
```
