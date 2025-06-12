# HTTPS Agent Configuration

```rust
use std::sync::Arc;
use reqwest::ClientBuilder;

use binance_sdk::crypto_loan;
use binance_sdk::config;

let custom_agent = config::HttpAgent(Arc::new(|builder: ClientBuilder| {
    builder.danger_accept_invalid_certs(false)
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = crypto_loan::CryptoLoanRestApi::production(configuration);
let params = crypto_loan::rest_api::GetFlexibleLoanBorrowHistoryParams::default();
let response = client.get_flexible_loan_borrow_history(params).await?;
```
