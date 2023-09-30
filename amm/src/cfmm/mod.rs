//! Constant Function Market Makers (CFMM)
//! This includes
//! 1. Uniswap-style Constant Product Market Maker (CPMM)
//!
//!

use crate::PurchaseError;
pub mod cpmm;

pub trait ConstantFunctionMarketMaker {
    fn local_assets(&self) -> &[f64];
    fn local_assets_mut(&mut self) -> &mut[f64];

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
