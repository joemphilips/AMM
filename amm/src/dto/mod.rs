use serde;

#[derive(Clone, PartialEq, Debug, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketId(u64);

#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct GetPriceForPurchase {
    pub purchase_vector: Vec<f64>,
}

#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct GetPricesAtThePoint {}

#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct Purchase {
    pub purchase_vector: Vec<f64>,
}
