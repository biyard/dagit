use std::str::FromStr;

use bdk::prelude::*;

#[api_model(base = "/v1/artwork_prices", table = artwork_prices)]
pub struct ArtworkPrice {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub amount: f64,

    #[api_model(summary, action = create)]
    pub currency: Currency,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Currency {
    #[default]
    NaN = 1,
    ETH = 2,
    BTC = 3,
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ETH" => Ok(Currency::ETH),
            "BTC" => Ok(Currency::BTC),
            _ => Err(format!("Unknown currency: {}", s)),
        }
    }
}
impl Currency {
    pub fn iter() -> impl Iterator<Item = Currency> {
        [Currency::ETH, Currency::BTC].iter().copied()
    }
}
