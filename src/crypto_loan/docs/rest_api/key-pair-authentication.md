# Key Pair Based Authentication

```rust
use binance_sdk::crypto_loan;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = crypto_loan::CryptoLoanRestApi::production(configuration);
let params = crypto_loan::rest_api::GetFlexibleLoanBorrowHistoryParams::default();
let response = client.get_flexible_loan_borrow_history(params).await?;
```
