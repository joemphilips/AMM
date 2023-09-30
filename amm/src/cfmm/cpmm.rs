
use crate::{cfmm::ConstantFunctionMarketMaker, PurchaseError, AssetInfo, AssetId, utils::FinitePositiveFloat};

use super::{OrderInfo, Error as CFMMError};

/// `ConstantProductMarketMaker` was originally used in Uniswap V2.
/// It has following advantages
/// 1. users can dynamically crowdfund an asset to trade
/// 2. Amount of the trade is bounded, so the server can never be out of funds.
#[derive(Debug, Clone)]
pub struct ConstantProductMarketMaker {
    local_asset_1: AssetInfo,
    local_asset_2: AssetInfo
}

impl ConstantProductMarketMaker {
    pub fn k(&self) -> f64 {
        self.local_asset_1.amount.inner() * self.local_asset_2.amount.inner()
    }

    pub fn asset_by_id(&self, id: &AssetId) -> Result<&AssetInfo, CFMMError> {
        if &self.local_asset_1.id == id { Ok(&self.local_asset_1) }
        else if &self.local_asset_2.id == id { Ok(&self.local_asset_2) }
        else { Err(CFMMError::UnknownAssetId) }
    }

    fn asset_by_id_mut(&mut self, id: &AssetId) -> Result<&mut AssetInfo, CFMMError> {
        if &self.local_asset_1.id == id { Ok(&mut self.local_asset_1) }
        else if &self.local_asset_2.id == id { Ok(&mut self.local_asset_2) }
        else { Err(CFMMError::UnknownAssetId) }
    }

    /// Returns amount of token which this market maker can offer according
    /// to the order.
    /// Returns error for unknown asset id.
    pub fn price_for_order(&self, order: &OrderInfo) -> Result<f64, CFMMError> {
        let k = self.k();
        let amount_before = self.asset_by_id(&order.id)?.amount.inner();
        let y = k / (
            if order.is_buy() { amount_before - order.amount }
            else { amount_before + order.amount }
        );
        Ok(y)
    }

    fn is_quoted_by_asset1(&self, order: &OrderInfo) -> bool {
        if order.is_buy() {
            &self.local_asset_1.id == &order.id
        } else {
            &self.local_asset_1.id != &order.id
        }
    }

    pub fn fund(&mut self, asset_info: &AssetInfo) -> Result<(), CFMMError> {
        let asset = self.asset_by_id_mut(&asset_info.id)?;
        asset.amount.0 += asset_info.amount.0;
        Ok(())
    }

    pub fn price(&self) -> FinitePositiveFloat {
        self.local_asset_1.amount / self.local_asset_2.amount
    }

    /// Move the internal reserves, and returns the amount that user will get.
    pub fn order(&mut self, order: &OrderInfo) -> Result<f64, CFMMError> {
        let amount_y = self.price_for_order(&order)?;
        if self.is_quoted_by_asset1(&order) {
            self.local_asset_1.amount.0 -= order.amount;
            self.local_asset_2.amount.0 += amount_y;
        } else {
            self.local_asset_1.amount.0 += order.amount;
            self.local_asset_2.amount.0 -= amount_y;
        }

        Ok(amount_y)
    }
}
