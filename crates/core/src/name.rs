use ethabi::{AbiError, Event, Function};

/// A trait for ABI items that have a name.
///
/// NOTE: Just an utility trait to avoid code duplication.
pub(crate) trait HasName {
    fn name(&self) -> &str;
}

impl HasName for Function {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasName for Event {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasName for AbiError {
    fn name(&self) -> &str {
        &self.name
    }
}
