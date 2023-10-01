
use noisy_float::types::R64;

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
    fn k(&self) -> R64 {
        self.local_asset_1.amount * self.local_asset_2.amount
    }

    fn is_quoted_by_asset1(&self, order: &OrderInfo) -> bool {
        if order.is_buy() {
            &self.local_asset_1.id == &order.id
        } else {
            &self.local_asset_1.id != &order.id
        }
    }

    pub fn price(&self) -> f64 {
        (self.local_asset_1.amount / self.local_asset_2.amount).into()
    }

}


impl ConstantFunctionMarketMaker for ConstantProductMarketMaker {
    fn local_asset_1(&self) -> &AssetInfo {
        &self.local_asset_1
    }

    fn local_asset_2(&self) -> &AssetInfo {
        &self.local_asset_2
    }
    fn local_asset_1_mut(&mut self) -> &mut AssetInfo {
        &mut self.local_asset_1
    }

    fn local_asset_2_mut(&mut self) -> &mut AssetInfo {
        &mut self.local_asset_2
    }

    fn price_for_order(&self, order: &OrderInfo) -> f64 {
        let k = self.k();
        let amount_before = self.asset_by_index(order.index).amount;
        let y = k / (
            if order.is_buy() { amount_before - order.amount }
            else { amount_before + order.amount }
        );
        y.into()
    }

    /// Move the internal reserves, and returns the amount that user will get.
    fn order(&mut self, order: &OrderInfo) -> f64 {
        let amount_y = self.price_for_order(&order);
        if self.is_quoted_by_asset1(&order) {
            self.local_asset_1.amount -= order.amount;
            self.local_asset_2.amount += amount_y;
        } else {
            self.local_asset_1.amount += order.amount;
            self.local_asset_2.amount -= amount_y;
        }

        amount_y
    }
}
