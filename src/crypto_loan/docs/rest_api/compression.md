# Compression Configuration

```rust
use binance_sdk::crypto_loan;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .compression(false) // default is true
    .build()?;

let client = crypto_loan::CryptoLoanRestApi::production(configuration);
let params = crypto_loan::rest_api::GetFlexibleLoanBorrowHistoryParams::default();
let response = client.get_flexible_loan_borrow_history(params).await?;
```
