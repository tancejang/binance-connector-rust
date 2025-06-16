# Keep-Alive Configuration

```rust
use binance_sdk::nft;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .keep_alive(false) // default is true
    .build()?;

let client = nft::NFTRestApi::production(configuration);
let params = nft::rest_api::GetNftAssetParams::default();
let response = client.get_nft_asset(params).await?;
```
