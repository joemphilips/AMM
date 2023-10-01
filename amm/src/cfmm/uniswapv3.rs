use noisy_float::prelude::*;

use crate::AssetInfo;

use super::ConstantFunctionMarketMaker;

pub struct UniswapV3MarketMaker {
    local_asset_1: AssetInfo,
    local_asset_2: AssetInfo,
}

impl ConstantFunctionMarketMaker for UniswapV3MarketMaker {
    fn base_asset(&self) -> &AssetInfo {
        &self.local_asset_1
    }

    fn quote_asset(&self) -> &AssetInfo {
        &self.local_asset_2
    }
    fn base_asset_mut(&mut self) -> &mut AssetInfo {
        &mut self.local_asset_1
    }

    fn quote_asset_mut(&mut self) -> &mut AssetInfo {
        &mut self.local_asset_2
    }

    fn price_for_order(&self, order: &super::OrderInfo) -> f64 {
        todo!()
    }

    fn order(&mut self, order: &super::OrderInfo) -> f64 {
        todo!()
    }
}
