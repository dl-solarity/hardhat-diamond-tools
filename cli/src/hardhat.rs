//! This module provides some stuff related to Hardhat compiled contracts.

use ethabi::Contract as Abi;

#[derive(Debug, serde::Deserialize)]
pub struct HarhatAbi {
    pub abi: Abi,
}
