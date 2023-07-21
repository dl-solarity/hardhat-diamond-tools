use std::collections::BTreeSet;

use ethabi::Function;

#[derive(Debug)]
pub enum IncludeExcludeFilter {
    /// Names of the methods that will be included in the merged ABI.
    /// If empty, all methods will be included. If not empty, only methods
    /// with names in this list will be included.
    ///
    /// By default, all methods are included.
    Include(BTreeSet<String>),
    /// Names of the methods that will be excluded from the merged ABI.
    ///
    /// By default, all methods are included.
    Exclude(BTreeSet<String>),
}

impl Default for IncludeExcludeFilter {
    fn default() -> Self {
        Self::Include(BTreeSet::new())
    }
}

impl IncludeExcludeFilter {
    pub fn filter(&self, methods: &[Function]) -> bool {
        match self {
            Self::Include(filter_set) => {
                filter_set.is_empty() || is_in_filter_set(filter_set, methods)
            }
            Self::Exclude(filter_set) => !is_in_filter_set(filter_set, methods),
        }
    }
}

/// Checks if the given methods are in the given filter set.
///
/// If there is only one method, it is checked by name. If there are more than
/// one method, they are checked by signature.
fn is_in_filter_set(filter_set: &BTreeSet<String>, methods: &[Function]) -> bool {
    if methods.len() == 1 {
        return filter_set.contains(&methods[0].name);
    }

    let methods = BTreeSet::from_iter(methods.iter().map(|method| method.signature()));

    methods.intersection(filter_set).count() == 1
}

#[cfg(test)]
mod tests {
    use ethabi::{Function, Param, ParamType};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref FOO_1: Function = Function {
            name: "foo".to_owned(),
            inputs: vec![Param {
                name: "foo1".to_owned(),
                kind: ParamType::Address,
                internal_type: None,
            }],
            outputs: vec![],
            #[allow(deprecated)]
            constant: None,
            state_mutability: ethabi::StateMutability::NonPayable,
        };
        static ref FOO_2: Function = Function {
            name: "foo".to_owned(),
            inputs: vec![
                Param {
                    name: "foo1".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "foo2".to_owned(),
                    kind: ParamType::Address,
                    internal_type: None,
                }
            ],
            outputs: vec![],
            #[allow(deprecated)]
            constant: None,
            state_mutability: ethabi::StateMutability::NonPayable,
        };
    }

    mod is_in_filter {
        use std::collections::BTreeSet;

        use crate::filter::{
            is_in_filter_set,
            tests::{FOO_1, FOO_2},
        };

        #[test]
        fn test_by_name() {
            let filter_set = BTreeSet::from_iter(vec![FOO_1.name.clone(), "bar".to_owned()]);

            let methods = vec![FOO_1.clone()];

            assert!(
                is_in_filter_set(&filter_set, &methods),
                "Should find `foo` method in the filter set by name"
            );
        }

        #[test]
        fn test_by_signature() {
            let filter_set = BTreeSet::from_iter(vec![FOO_1.name.clone(), FOO_2.signature()]);

            let methods = vec![FOO_1.clone(), FOO_2.clone()];

            assert!(
                is_in_filter_set(&filter_set, &methods),
                "Should find `foo` method in the filter set by signature"
            );
        }
    }
}
