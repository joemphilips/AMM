pub mod lmsr;
pub mod ls_lmsr;
pub mod lsmr_logsumexp;

use crate::cfmm::Error as CFMMError;

use amplify::{Display, Error, From};

/// Purchase smaller than this will be considered as 0.
pub const MINIMAL_PURCHASE: f64 = 0.000001;

#[derive(Clone, Debug, PartialEq, Eq, Display, Error, From)]
#[display(doc_comments)]
pub enum AMMError {
    /// An ontract requires at least two outcomes
    OutcomeLessThanTwo,
    /// Liquidity parameter is either Nan, infinite, 0, Negative
    BogusLiquidityParam,
    /// Error when tried to purchase some securities
    PurchaseError(PurchaseError),
    /// Error for funding the CFMM.
    FundingError(CFMMError),
}

#[derive(Clone, Debug, PartialEq, Eq, Display, Error, From)]
#[display(doc_comments)]
pub enum PurchaseError {
    /// tried to purchase negative amount
    NegativePurchase,

    /// Tried to purchase bogus amount (e.g. NaN, Infinite)
    NonNormalPurchase,

    /// Tried to purchase (almost) nothing.
    TooSmall,

    /// Must give an purchase vector which has the exactly the same length
    /// with number of possible outcomes.
    WrongPurchaseLength,

    /// An user tried to purchase an asset with the same asset.
    CannotPurchaseWithSameAsset,
}
fn is_fine_purchase(purchase_vector: &[f64]) -> Result<(), PurchaseError> {
    let mut all_zero = true;
    for p in purchase_vector {
        if all_zero {
            all_zero = p.abs() < MINIMAL_PURCHASE;
        }
        if p.is_nan() || p.is_infinite() {
            return Err(PurchaseError::NonNormalPurchase);
        }
        if p.is_sign_negative() {
            return Err(PurchaseError::NegativePurchase);
        }
    }
    if all_zero {
        return Err(PurchaseError::TooSmall);
    }
    Ok(())
}

/// Market Maker created from particular cost-funcitn, e.g. Hanson's LMSR
/// This is a classic example of AMM for prediction market and it has been
/// studied for fair amount of time.
pub trait CostFunctionMarketMaker {
    /// Total securities issued so far
    fn total_securities(&self) -> &[f64];
    fn total_securities_mut(&mut self) -> &mut [f64];
    fn cost_function(&self) -> f64;
    fn price_for_purchase(&self, purchase_vector: &[f64]) -> f64;
    fn price_for_showing(&self, security_index: usize) -> f64;
    fn bounded_loss(&self) -> Option<f64>;
    fn odds(&self) -> Vec<f64> {
        let total = self.total_securities();
        let sum: f64 = total.iter().sum();
        total.iter().map(|x| x / sum).collect()
    }
    fn purchase(&mut self, purchase_vector: &[f64]) -> Result<(), PurchaseError> {
        is_fine_purchase(purchase_vector)?;
        let total_securities = self.total_securities_mut();
        if total_securities.len() != purchase_vector.len() {
            return Err(PurchaseError::WrongPurchaseLength);
        }
        for (s, p) in total_securities.iter_mut().zip(purchase_vector) {
            *s += p;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::lmsr::LMScoringRule as LMSR;
    use super::ls_lmsr::LSLMScoringRule;
    use super::lsmr_logsumexp::LMScoringRule as LogSumExpLMSR;
    use super::{is_fine_purchase, AMMError, CostFunctionMarketMaker};
    use proptest::prelude::*;

    fn approx_equal(a: f64, b: f64, diff: f64) -> bool {
        a - b < diff || b - a < diff
    }
    macro_rules! prop_assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            prop_assert!($x - $y < $d || $y - $x < $d)
        };
    }

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    fn arb_purchase_scalar() -> impl Strategy<Value = f64> {
        any::<f64>().prop_filter_map("Purchase vector must be non-zero sane value", |q| {
            if !q.is_nan() && !q.is_infinite() {
                Some(q.abs())
            } else {
                None
            }
        })
    }

    fn arb_purchase_vector(max_size: usize) -> impl Strategy<Value = Vec<f64>> {
        proptest::collection::vec(arb_purchase_scalar(), 0..max_size)
            .prop_filter("Must be fine purchase", |v| is_fine_purchase(v).is_ok())
    }

    fn arb_purchase_vectors(
        dimension: usize,
        max_size: usize,
    ) -> impl Strategy<Value = Vec<Vec<f64>>> {
        let arb_vector = proptest::collection::vec(arb_purchase_scalar(), dimension)
            .prop_filter("Must be fine purchase", |v| is_fine_purchase(v).is_ok());
        proptest::collection::vec(arb_vector, 1..max_size)
    }

    fn arb_liquidity_param() -> impl Strategy<Value = f64> {
        any::<f64>().prop_filter("must be positive normal number", |b| {
            b.is_normal() && !b.is_sign_negative()
        })
    }

    proptest! {
        #[test]
        fn two_lmsr_must_return_same_results(purchase_vector in arb_purchase_vector(12), liquidity_param in arb_liquidity_param()) {
            let r1 = LMSR::try_create(purchase_vector.len(), liquidity_param.abs());
            let r2 = LogSumExpLMSR::try_create(purchase_vector.len(), liquidity_param.abs());
            match (r1, r2) {
                (Ok(ref mut msr_1), Ok(ref mut msr_2)) => {
                    msr_1.purchase(&purchase_vector).unwrap();
                    msr_2.purchase(&purchase_vector).unwrap();
                    prop_assert_delta!(msr_1.cost_function(), msr_2.cost_function(), 0.00000001)
                },
                (Err(ref e1), Err(ref e2)) => {
                    prop_assert_eq!(e1.clone(), e2.clone())
                },
                (ref r1, ref r2) => {
                    prop_assert!(false, "r1: {:?}, r2: {:?}", r1, r2)
                }
            }
        }
    }

    fn get_all_marketmakers(dimension: usize, param: f64) -> Vec<Box<dyn CostFunctionMarketMaker>> {
        vec![
            Box::new(LMSR::try_create(dimension, param).unwrap()),
            Box::new(LogSumExpLMSR::try_create(dimension, param).unwrap()),
            Box::new(LSLMScoringRule::try_create(dimension, param).unwrap()),
        ]
    }

    proptest! {
        #[test]
        fn odds_total_must_be_1(purchase_vectors in arb_purchase_vectors(3, 12), param in arb_liquidity_param()) {
            prop_assume!(param > 0.01);
            let msrs = get_all_marketmakers(purchase_vectors[0].len(), param);
            for mut msr in msrs {
                for p in &purchase_vectors {
                    msr.purchase(&p).unwrap();
                }
                let o: f64 = msr.odds().into_iter().sum();
                prop_assert_delta!(o, 1., 0.00000000001)
            }
        }
    }

    #[test]
    fn must_reject_wrong_purchase_vector() {
        let msrs = get_all_marketmakers(2, 10.);
        for mut msr in msrs {
            for bogus_purchase_vector in [vec![1., 1., 0.], vec![1.]] {
                let r = msr.purchase(&bogus_purchase_vector);
                assert!(r.is_err());
            }
        }
    }

    #[test]
    fn must_reject_too_small_liquidity() {
        let liquidity = 7.212815578282739e-276;
        let (r1, r2, r3) = (
            LMSR::try_create(2, liquidity),
            LogSumExpLMSR::try_create(2, liquidity),
            LSLMScoringRule::try_create(2, liquidity),
        );
        assert_eq!(r1.unwrap_err(), AMMError::BogusLiquidityParam);
        assert_eq!(r2.unwrap_err(), AMMError::BogusLiquidityParam);
        assert_eq!(r3.unwrap_err(), AMMError::BogusLiquidityParam);
        let liquidity = 0.02;
        let (r1, r2, r3) = (
            LMSR::try_create(2, liquidity),
            LogSumExpLMSR::try_create(2, liquidity),
            LSLMScoringRule::try_create(2, liquidity),
        );
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());
    }

    #[test]
    fn unittest1() {
        let test_cases = [(vec![1.0, 736.0], 20.)];

        for (purchase_vector, liquidity) in test_cases {
            let mut msr1 = LMSR::try_create(2, liquidity).unwrap();
            let mut msr2 = LogSumExpLMSR::try_create(2, liquidity).unwrap();
            msr1.purchase(&purchase_vector).unwrap();
            msr2.purchase(&purchase_vector).unwrap();

            let c1 = msr1.cost_function();
            let c2 = msr2.cost_function();
            print!("c1: {:?}, c2: {:?}", c1, c2);
            assert_delta!(c1, c2, 0.00000001);
        }
    }
}
