//! Constant Function Market Makers (CFMM)
//! This includes
//! 1. UniswapV2-style Constant Product Market Maker (CPMM)
//!
//!

use amplify::{Display, Error, From};

use crate::{PurchaseError, AssetId};
pub mod cpmm;

/// Error when user tries to fund the AMM>
#[derive(Clone, Debug, PartialEq, Eq, Display, Error, From)]
#[display(doc_comments)]
pub enum Error {
    /// Type of assets does not match those of AMM.
    InvalidAssetCount,

    /// Unknown Asset Id
    UnknownAssetId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderType {
  Buy,
  Sell,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrderInfo {
  id: AssetId,
  amount: f64,
  order_type: OrderType
}
impl OrderInfo {
  pub fn is_buy(&self) -> bool { self.order_type == OrderType::Buy }
}

impl OrderInfo {
  fn try_create(id: AssetId, amount: f64, order_type: OrderType) -> Result<Self, PurchaseError> {
      if amount.is_nan() || amount.is_infinite() {
          return Err(PurchaseError::NonNormalPurchase);
      }
      if amount.is_sign_negative() {
          return Err(PurchaseError::NegativePurchase);
      }
      Ok(Self { id, amount, order_type })
  }
}


pub trait ConstantFunctionMarketMaker {
    fn local_assets(&self) -> &[f64];
    fn local_assets_mut(&mut self) -> &mut [f64];

    fn fund(&mut self, fund_vector: &[f64]) -> Result<(), Error> {
        let assets = self.local_assets_mut();
        if fund_vector.len() != assets.len() {
            return Err(Error::InvalidAssetCount);
        }
        for (a, f) in assets.iter_mut().zip(fund_vector) {
            *a += f;
        }

        Ok(())
    }

    fn price_for_purchase(&self, order_info: &OrderInfo) -> f64;

    fn purchase(&mut self, order_info: &OrderInfo, index_to_buy: usize) -> Result<f64, PurchaseError>;
}
