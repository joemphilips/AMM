use super::{
    lmsr::{cost_function_md, price_for_purchase, price_for_showing},
    AMMError, CostFunctionMarketMaker,
};

/// `b` value must have certain amount for sane numerical computing
pub const MINIMAL_LIQUIDITY_A: f64 = 0.0001;

#[derive(Debug, Clone)]
pub struct LSLMScoringRule {
    total_securities: Vec<f64>,
    alpha: f64,
}

impl LSLMScoringRule {
    pub fn try_create(num_outcomes: usize, alpha: f64) -> Result<Self, AMMError> {
        if num_outcomes <= 1 {
            Err(AMMError::OutcomeLessThanTwo)
        } else if !alpha.is_normal() || alpha.is_sign_negative() {
            Err(AMMError::BogusLiquidityParam)
        } else if alpha < MINIMAL_LIQUIDITY_A {
            Err(AMMError::BogusLiquidityParam)
        } else {
            Ok(Self {
                total_securities: vec![0.; num_outcomes],
                alpha,
            })
        }
    }

    pub fn b(&self) -> f64 {
        &self.alpha * &self.total_securities.iter().sum::<f64>()
    }
}

impl CostFunctionMarketMaker for LSLMScoringRule {
    fn cost_function(&self) -> f64 {
        cost_function_md(&self.total_securities, self.b())
    }

    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64 {
        price_for_purchase(&self.total_securities, purchase_vector, self.b())
    }

    fn price_for_showing(&self, security_index: usize) -> f64 {
        price_for_showing(&self.total_securities, security_index, self.b())
    }

    fn total_securities(&self) -> &[f64] {
        self.total_securities.as_ref()
    }

    fn total_securities_mut(&mut self) -> &mut [f64] {
        self.total_securities.as_mut()
    }

    fn bounded_loss(&self) -> Option<f64> {
        Some((self.total_securities.len() as f64).ln() * self.b())
    }
}
