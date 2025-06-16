# Retries Configuration

```rust
use binance_sdk::vip_loan;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = vip_loan::VIPLoanRestApi::production(configuration);
let params = vip_loan::rest_api::GetCollateralAssetDataParams::default();
let response = client.get_collateral_asset_data(params).await?;
```
