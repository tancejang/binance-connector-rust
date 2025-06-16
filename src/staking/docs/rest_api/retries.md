# Retries Configuration

```rust
use binance_sdk::staking;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = staking::StakingRestApi::production(configuration);
let params = staking::rest_api::ClaimBoostRewardsParams::default();
let response = client.claim_boost_rewards(params).await?;
```
