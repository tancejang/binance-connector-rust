# Timeout

```rust
use binance_sdk::staking;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .timeout(5000)
    .build()?;

let client = staking::StakingRestApi::production(configuration);
let params = staking::rest_api::ClaimBoostRewardsParams::default();
let response = client.claim_boost_rewards(params).await?;
```
