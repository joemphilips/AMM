use noisy_float::types::R64;

use crate::{
    cfmm::ConstantFunctionMarketMaker, cost_function::PurchaseError, utils::FinitePositiveFloat,
    AssetId, AssetInfo,
};

use super::{Error as CFMMError, OrderInfo};

/// `ConstantProductMarketMaker` was originally used in Uniswap V2.
/// It has following advantages
/// 1. users can dynamically crowdfund an asset to trade
/// 2. Amount of the trade is bounded, so the server can never be out of funds.
#[derive(Debug, Clone)]
pub struct ConstantProductMarketMaker {
    base_asset: AssetInfo,
    quote_asset: AssetInfo,
}

impl ConstantProductMarketMaker {
    fn k(&self) -> R64 {
        self.base_asset.amount * self.quote_asset.amount
    }

    fn is_base_buy(&self, order: &OrderInfo) -> bool {
        if order.is_buy() {
            &self.base_asset.id == &order.id
        } else {
            &self.base_asset.id != &order.id
        }
    }

    pub fn price(&self) -> f64 {
        (self.base_asset.amount / self.quote_asset.amount).into()
    }
}

impl ConstantFunctionMarketMaker for ConstantProductMarketMaker {
    fn base_asset(&self) -> &AssetInfo {
        &self.base_asset
    }

    fn quote_asset(&self) -> &AssetInfo {
        &self.quote_asset
    }
    fn base_asset_mut(&mut self) -> &mut AssetInfo {
        &mut self.base_asset
    }

    fn quote_asset_mut(&mut self) -> &mut AssetInfo {
        &mut self.quote_asset
    }

    fn price_for_order(&self, order: &OrderInfo) -> f64 {
        let k = self.k();
        let amount_before = self.asset_by_index(order.index).amount;
        let y = k
            / (if order.is_buy() {
                amount_before - order.amount
            } else {
                amount_before + order.amount
            });
        y.into()
    }

    /// Move the internal reserves, and returns the amount that user will get.
    fn order(&mut self, order: &OrderInfo) -> f64 {
        let amount_y = self.price_for_order(&order);
        if self.is_base_buy(&order) {
            self.base_asset.amount -= order.amount;
            self.quote_asset.amount += amount_y;
        } else {
            self.base_asset.amount += order.amount;
            self.quote_asset.amount -= amount_y;
        }

        amount_y
    }
}
