//! This modile provides engine that implements all the logic.

use std::collections::BTreeMap;

use ethabi::{AbiError, Contract, Event, Function};

use crate::{
    filter::FunctionsFilter,
    name::HasName,
    signature::{HasSignature, Signature},
};

#[derive(Debug, Default)]
pub struct DiamondMerger<F: FunctionsFilter> {
    /// Names of the methods that will be included or excluded in
    /// or from the merged ABI.
    pub(crate) filter: F,
}

impl<F: FunctionsFilter> DiamondMerger<F> {
    pub fn new(filter: F) -> Self {
        Self { filter }
    }

    pub fn merge(&self, abis: Vec<Contract>) -> Contract {
        let (functions, events, errors) = flatten_contracts(abis);

        let filtered_functions = functions
            .into_iter()
            .filter(|(signature, func)| self.filter.filter_functions(signature, func))
            .collect::<BTreeMap<_, _>>();

        // NOTE(Velnbur): Add filters for events and errors if needed.

        unflatten_contract(filtered_functions, events, errors)
    }
}

/// Flattern contracts from one array to three maps.
fn flatten_contracts(
    abis: Vec<Contract>,
) -> (
    BTreeMap<Signature, Function>,
    BTreeMap<Signature, Event>,
    BTreeMap<Signature, AbiError>,
) {
    abis.into_iter()
        .map(|abi| {
            let functions = flatten_by_signature(abi.functions);
            let events = flatten_by_signature(abi.events);
            let errors = flatten_by_signature(abi.errors);

            (functions, events, errors)
        })
        .fold(
            (BTreeMap::new(), BTreeMap::new(), BTreeMap::new()),
            |mut acc, (functions, events, errors)| {
                acc.0.extend(functions);
                acc.1.extend(events);
                acc.2.extend(errors);

                acc
            },
        )
}

fn flatten_by_signature<T: HasSignature>(t: BTreeMap<Signature, Vec<T>>) -> BTreeMap<Signature, T> {
    t.into_iter()
        .map(|(_name, overrides)| {
            overrides
                .into_iter()
                .map(|func| (func.long_signature(), func))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<BTreeMap<_, _>>()
}

/// After some manipulations, return the contract to its original form.
fn unflatten_contract(
    functions: BTreeMap<Signature, Function>,
    events: BTreeMap<Signature, Event>,
    errors: BTreeMap<Signature, AbiError>,
) -> Contract {
    Contract {
        constructor: None,
        functions: unflatten_by_name(functions),
        events: unflatten_by_name(events),
        errors: unflatten_by_name(errors),
        receive: false,
        fallback: false,
    }
}

fn unflatten_by_name<T: HasName>(items: BTreeMap<Signature, T>) -> BTreeMap<String, Vec<T>> {
    let mut flattened = BTreeMap::new();

    for (_sig, item) in items {
        flattened
            .entry(item.name().to_owned())
            .or_insert_with(Vec::new)
            .push(item);
    }

    flattened
}
