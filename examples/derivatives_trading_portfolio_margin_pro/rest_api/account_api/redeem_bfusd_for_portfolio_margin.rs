use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin_pro::{
    DerivativesTradingPortfolioMarginProRestApi, rest_api::RedeemBfusdForPortfolioMarginParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the DerivativesTradingPortfolioMarginPro REST API client
    let rest_client = DerivativesTradingPortfolioMarginProRestApi::production(rest_conf);

    // Setup the API parameters
    let params = RedeemBfusdForPortfolioMarginParams::builder(
        "from_asset_example".to_string(),
        "target_asset_example".to_string(),
        dec!(1.0),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .redeem_bfusd_for_portfolio_margin(params)
        .await
        .context("redeem_bfusd_for_portfolio_margin request failed")?;

    info!(?response.rate_limits, "redeem_bfusd_for_portfolio_margin rate limits");
    let data = response.data().await?;
    info!(?data, "redeem_bfusd_for_portfolio_margin data");

    Ok(())
}
