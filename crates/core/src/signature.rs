use ethabi::{AbiError, Event, Function};

/// Signature of the function, event or error.
///
/// For functions and events it is the name and a list of types of the arguments.
/// For example:
///
/// - `foo(uint256, address)`
/// - `bar(string, bool)`
///
/// For errors it is the name of the error.
/// For example:
///
/// - `FooError`
/// - `BarError`
pub(crate) type Signature = String;

/// Some useful trait for converting items from ABI into signature.
///
/// Shouldn't be exposed as we are using it only internally.
pub(crate) trait HasSignature {
    fn long_signature(&self) -> Signature;
}

impl HasSignature for Function {
    fn long_signature(&self) -> Signature {
        let inputs = self
            .inputs
            .iter()
            .map(|input| input.kind.to_string())
            .collect::<Vec<_>>();

        format!("{}({})", self.name, inputs.join(","))
    }
}

impl HasSignature for Event {
    fn long_signature(&self) -> Signature {
        let inputs = self
            .inputs
            .iter()
            .map(|input| input.kind.to_string())
            .collect::<Vec<_>>();

        format!("{}({})", self.name, inputs.join(","))
    }
}

impl HasSignature for AbiError {
    fn long_signature(&self) -> String {
        self.name.clone()
    }
}
