//! Logarithmic market scoring rule

use std::f64::consts::E;

use super::{AMMError, CostFunctionMarketMaker};

/// `b` value must have certain amount for sane numerical computing
pub const MINIMAL_LIQUIDITY_B: f64 = 0.0001;

/// Multi dimensional cost function
pub(crate) fn cost_function_md(total_security: &[f64], b: f64) -> f64 {
    b * total_security
        .into_iter()
        .map(|q| E.powf(q / b))
        .sum::<f64>()
        .ln()
}

pub(crate) fn price_for_purchase(total_security: &[f64], purchase_vector: &[f64], b: f64) -> f64 {
    let mut total_security_after = Vec::with_capacity(total_security.len());
    for (i, q) in total_security.iter().enumerate() {
        total_security_after[i] = *q + purchase_vector[i];
    }
    cost_function_md(total_security_after.as_ref(), b) - cost_function_md(total_security, b)
}

/// Price of the specific security at certain time.
/// This is an special case of `price_for_purchase` function to purchase
/// Infinitely small amount of security.
/// And it is a partial derivatives of the cost function.
pub(crate) fn price_for_showing(total_security: &[f64], security_index: usize, b: f64) -> f64 {
    let l = |q: &f64| E.powf(q / b);
    l(&total_security[security_index]) / total_security.iter().map(l).sum::<f64>()
}

#[derive(Debug, Clone)]
pub struct LMScoringRule {
    total_securities: Vec<f64>,
    liquidity: f64,
}

impl LMScoringRule {
    pub fn try_create(outcomes: usize, liquidity: f64) -> Result<Self, AMMError> {
        if outcomes <= 1 {
            Err(AMMError::OutcomeLessThanTwo)
        } else if !liquidity.is_normal() || liquidity.is_sign_negative() {
            Err(AMMError::BogusLiquidityParam)
        } else if liquidity < MINIMAL_LIQUIDITY_B {
            Err(AMMError::BogusLiquidityParam)
        } else {
            Ok(Self {
                total_securities: vec![0.; outcomes],
                liquidity,
            })
        }
    }
}

impl CostFunctionMarketMaker for LMScoringRule {
    fn cost_function(&self) -> f64 {
        cost_function_md(&self.total_securities.as_ref(), self.liquidity)
    }

    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64 {
        price_for_purchase(&self.total_securities, purchase_vector, self.liquidity)
    }

    fn price_for_showing(&self, security_index: usize) -> f64 {
        price_for_showing(&self.total_securities, security_index, self.liquidity)
    }

    fn total_securities(&self) -> &[f64] {
        self.total_securities.as_ref()
    }
    fn total_securities_mut(&mut self) -> &mut [f64] {
        self.total_securities.as_mut()
    }

    fn bounded_loss(&self) -> Option<f64> {
        Some((self.total_securities.len() as f64).ln() * self.liquidity)
    }
}
