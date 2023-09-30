use crate::MarketScoringRule;



/// Constant Product Market Maker.
/// It was originally used in Uniswap.
/// It has following advantages
/// 1. users can dynamically crowdfund an asset to trade
/// 2. Amount of the trade is bounded, so server can never be out of funds.

#[derive(Debug, Clone)]
pub struct ConstantProductMarketScoringRule {}


impl MarketScoringRule for ConstantProductMarketScoringRule  {
    fn total_securities(&self) -> &[f64] {
        todo!()
    }

    fn total_securities_mut(&mut self) -> &mut [f64] {
        todo!()
    }

    fn cost_function(&self) -> f64 {
        todo!()
    }

    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64 {
        todo!()
    }

    fn price_for_showing(&self, security_index: usize) -> f64 {
        todo!()
    }

    fn bounded_loss(&self) -> Option<f64> {
        todo!()
    }
}
