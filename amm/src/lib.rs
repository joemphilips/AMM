//! Automated Market Maker (AMM) for Rust.
//!
//! This crate includes types/funcitons/traits to build automated market maker.
//! AMMs are categorized into 2 indipendent modules.
//!
//! ### 1. Cost Function based market maker (`crate::cost_function`)
//!
//! This is primarily for Prediction Market.
//! e.g. Hanson's LMSR.
//!
//! It is useful if you are the one issuing the token, and the token
//! will be redeemed at the certain point of time in future.
//!
//! ### 2. Constant Function Market Maker (`crate::cfmm`)
//!
//! This category of market maker is for providing a liquidity for tokens.
//! It doesn't matter if those tokens are issued by someone else, or by
//! yourself.
//! It has been mostly researched in the context of DeFi (Decentralized
//! Finance).
//!
pub mod cfmm;
pub mod cost_function;
pub mod utils;

pub mod dto;
pub mod entity;

use amplify::{From, Wrapper};
use noisy_float::types::R64;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Wrapper)]
pub struct AssetId([u8; 32]);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AssetInfo {
    id: AssetId,
    amount: R64,
    ticker: String,
}
