use crate::cfmm::ConstantFunctionMarketMaker;
use crate::PurchaseError;

/// Constant Product Market Maker.
/// It was originally used in Uniswap.
/// It has following advantages
/// 1. users can dynamically crowdfund an asset to trade
/// 2. Amount of the trade is bounded, so server can never be out of funds.

#[derive(Debug, Clone)]
pub struct ConstantProductMarketScoringRule {
    /// Assets we have in local.
    local_assets: Vec<f64>,
}

impl ConstantProductMarketScoringRule {}

impl ConstantFunctionMarketMaker for ConstantProductMarketScoringRule {
    fn local_assets(&self) -> &[f64] {
        self.local_assets.as_ref()
    }

    fn local_assets_mut(&mut self) -> &mut [f64] {
        self.local_assets.as_mut()
    }
}
