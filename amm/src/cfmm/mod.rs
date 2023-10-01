//! Constant Function Market Makers (CFMM)
//! Also known as trading-function based market maker.
//! This includes
//! 1. UniswapV2-style Constant Product Market Maker (CPMM)
//!
//!

use amplify::{Display, Error, From};
use noisy_float::types::R64;

use crate::{AssetId, AssetInfo};
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
    /// a.k.a. "Base" asset
    Zero,
    /// a.k.a. "Quote" asset
    One,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OrderInfo {
    index: AssetIndex,
    id: AssetId,
    amount: R64,
    order_type: OrderType,
}

impl OrderInfo {
    pub fn is_buy(&self) -> bool {
        self.order_type == OrderType::Buy
    }
}

impl OrderInfo {
    pub fn new(index: AssetIndex, id: AssetId, amount: R64, order_type: OrderType) -> Self {
        Self {
            index,
            id,
            amount,
            order_type,
        }
    }
}

pub trait ConstantFunctionMarketMaker {
    fn base_asset(&self) -> &AssetInfo;
    fn quote_asset(&self) -> &AssetInfo;
    fn base_asset_mut(&mut self) -> &mut AssetInfo;
    fn quote_asset_mut(&mut self) -> &mut AssetInfo;

    fn asset_by_id(&self, id: &AssetId) -> Result<&AssetInfo, Error> {
        let one = self.base_asset();
        let two = self.quote_asset();
        if one.id == *id {
            Ok(one)
        } else if two.id == *id {
            Ok(two)
        } else {
            Err(Error::UnknownAssetId)
        }
    }

    fn asset_by_index(&self, index: AssetIndex) -> &AssetInfo {
        match index {
            AssetIndex::Zero => &self.base_asset(),
            AssetIndex::One => &self.quote_asset(),
        }
    }

    fn asset_by_id_mut(&mut self, id: &AssetId) -> Result<&mut AssetInfo, Error> {
        if &self.base_asset().id == id {
            Ok(self.base_asset_mut())
        } else if &self.quote_asset().id == id {
            Ok(self.quote_asset_mut())
        } else {
            Err(Error::UnknownAssetId)
        }
    }

    fn asset_by_index_mut(&mut self, index: AssetIndex) -> &mut AssetInfo {
        match index {
            AssetIndex::Zero => self.base_asset_mut(),
            AssetIndex::One => self.quote_asset_mut(),
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
