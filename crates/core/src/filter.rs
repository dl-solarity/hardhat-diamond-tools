use std::collections::BTreeSet;

use ethabi::Function;

pub trait FunctionsFilter {
    fn filter_functions(&self, signature: &str, func: &Function) -> bool;
}

impl FunctionsFilter for Box<dyn FunctionsFilter> {
    fn filter_functions(&self, sign: &str, func: &Function) -> bool {
        self.as_ref().filter_functions(sign, func)
    }
}

#[derive(Debug, serde::Deserialize)]
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

impl FunctionsFilter for IncludeExcludeFilter {
    fn filter_functions(&self, signature: &str, func: &Function) -> bool {
        self.filter(signature, func)
    }
}

impl IncludeExcludeFilter {
    pub fn filter(&self, signature: &str, func: &Function) -> bool {
        match self {
            Self::Include(filter_set) => {
                filter_set.is_empty() || is_in_filter_set(filter_set, signature, func)
            }
            Self::Exclude(filter_set) => !is_in_filter_set(filter_set, signature, func),
        }
    }

    pub fn from_include(include: Vec<String>) -> Self {
        Self::Include(include.into_iter().collect())
    }

    pub fn from_exclude(exclude: Vec<String>) -> Self {
        Self::Exclude(exclude.into_iter().collect())
    }
}

/// Check if the function is in the filter set.
///
/// The function is in the filter set if the signature or the name of the function is in the filter set.
fn is_in_filter_set(filter_set: &BTreeSet<String>, signature: &str, func: &Function) -> bool {
    filter_set
        .iter()
        .find(|filter| filter.as_str() == signature || filter.as_str() == &func.name)
        .is_some()
}
