# Retries Configuration

```rust
use binance_sdk::sub_account;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .retries(5) // Retry up to 5 times
    .backoff(2000) // 2 seconds between retries
    .build()?;

let client = sub_account::SubAccountRestApi::production(configuration);
let params = sub_account::rest_api::GetSummaryOfSubAccountsMarginAccountParams::default();
let response = client.get_summary_of_sub_accounts_margin_account(params).await?;
```
