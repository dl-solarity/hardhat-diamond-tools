//! This modile provides engine that implements all the logic.

use std::collections::{BTreeMap, BTreeSet};

use ethabi::Contract;

use crate::filter::IncludeExcludeFilter;

#[derive(Debug, Default)]
pub struct Engine {
    /// ABIs of the contracts that will be merged.
    pub(crate) abis: Vec<Contract>,
    /// Names of the methods that will be included or excluded in
    /// or from the merged ABI.
    pub(crate) filter: IncludeExcludeFilter,
    /// Resulting ABI.
    pub(crate) result: Contract,
}

impl Engine {
    pub fn new(abis: Vec<Contract>) -> Self {
        Self {
            abis,
            ..Default::default()
        }
    }

    pub fn with_include(mut self, include: Vec<String>) -> Self {
        self.filter = IncludeExcludeFilter::Include(BTreeSet::from_iter(include));
        self
    }

    pub fn with_exclude(mut self, exclude: Vec<String>) -> Self {
        self.filter = IncludeExcludeFilter::Exclude(BTreeSet::from_iter(exclude));
        self
    }

    pub fn merge(&mut self) {
        self.result = self.abis.iter().fold(Contract::default(), |mut acc, abi| {
            let functions = abi
                .functions
                .clone()
                .into_iter()
                .filter(|(_, funcs)| self.filter.filter(funcs))
                .collect::<BTreeMap<_, _>>();

            acc.functions.extend(functions);
            acc.events.extend(abi.events.clone());
            acc.errors.extend(abi.errors.clone());
            acc
        });
    }

    pub fn finish(self) -> Contract {
        self.result
    }
}

/* Some definitions required for wasm-bindgen */
impl Engine {}
