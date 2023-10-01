//! Constant Function Market Makers (CFMM)
//! Also known as trading-function based market maker.
//! This includes
//! 1. UniswapV2-style Constant Product Market Maker (CPMM)
//!
//!

use amplify::{Display, Error, From};
use noisy_float::types::R64;

use crate::{PurchaseError, AssetId, AssetInfo};
pub mod cpmm;
pub mod uniswapv3;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetIndex {
  One,
  Two
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrderInfo {
  index: AssetIndex,
  id: AssetId,
  amount: R64,
  order_type: OrderType
}
impl OrderInfo {
  pub fn is_buy(&self) -> bool { self.order_type == OrderType::Buy }
}

impl OrderInfo {
  pub fn new(index: AssetIndex, id: AssetId, amount: R64, order_type: OrderType) -> Self {
      Self { index, id, amount, order_type }
  }
}


pub trait ConstantFunctionMarketMaker {
    fn local_asset_1(&self) -> &AssetInfo;
    fn local_asset_2(&self) -> &AssetInfo;
    fn local_asset_1_mut(&mut self) -> &mut AssetInfo;
    fn local_asset_2_mut(&mut self) -> &mut AssetInfo;

    fn asset_by_id(&self, id: &AssetId) -> Result<&AssetInfo, Error> {
        let one = self.local_asset_1();
        let two = self.local_asset_2();
        if one.id == *id { Ok(one) }
        else if two.id == *id { Ok(two) }
        else { Err(Error::UnknownAssetId) }
    }

    fn asset_by_index(&self, index: AssetIndex) -> &AssetInfo {
      match index {
        AssetIndex::One => &self.local_asset_1(),
        AssetIndex::Two => &self.local_asset_2()
      }
    }

    fn asset_by_id_mut(&mut self, id: &AssetId) -> Result<&mut AssetInfo, Error> {
        if &self.local_asset_1().id == id { Ok(self.local_asset_1_mut()) }
        else if &self.local_asset_2().id == id { Ok(self.local_asset_2_mut()) }
        else { Err(Error::UnknownAssetId) }
    }

    fn asset_by_index_mut(&mut self, index: AssetIndex) -> &mut AssetInfo {
      match index {
        AssetIndex::One => self.local_asset_1_mut(),
        AssetIndex::Two => self.local_asset_2_mut()
      }
    }

    fn fund(&mut self, asset_info: &AssetInfo) -> Result<(), Error> {
        let asset = self.asset_by_id_mut(&asset_info.id)?;
        asset.amount += asset_info.amount;
        Ok(())
    }

    /// Returns an amount of token which this market maker can offer according
    /// to the order.
    /// Returns error for unknown asset id.
    fn price_for_order(&self, order: &OrderInfo) -> f64;

    fn order(&mut self, order: &OrderInfo) -> f64;

}
