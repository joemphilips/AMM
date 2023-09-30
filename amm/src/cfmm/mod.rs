//! Constant Function Market Makers (CFMM)
//! This includes
//! 1. Uniswap-style Constant Product Market Maker (CPMM)
//!
//!

use amplify::{Display, Error, From};

use crate::PurchaseError;
pub mod cpmm;

/// Error when user tries to fund the AMM>
#[derive(Clone, Debug, PartialEq, Eq, Display, Error, From)]
#[display(doc_comments)]
pub enum FundingError {
    /// Type of assets does not match those of AMM.
    InvalidAssetCount,
}

pub trait ConstantFunctionMarketMaker {
    fn local_assets(&self) -> &[f64];
    fn local_assets_mut(&mut self) -> &mut [f64];

    fn fund(&mut self, fund_vector: &[f64]) -> Result<(), FundingError> {
        let assets = self.local_assets_mut();
        if fund_vector.len() != assets.len() {
            return Err(FundingError::InvalidAssetCount);
        }
        for (a, f) in assets.iter_mut().zip(fund_vector) {
            *a += f;
        }
        Ok(())
    }

    fn purchase(&mut self, index: usize, amount: f64) -> Result<(), PurchaseError> {
        if amount.is_nan() || amount.is_infinite() {
            return Err(PurchaseError::NonNormalPurchase);
        }
        if amount.is_sign_negative() {
            return Err(PurchaseError::NegativePurchase);
        }
        let local_assets = self.local_assets_mut();

        local_assets[index] -= amount;

        Ok(())
    }
}
