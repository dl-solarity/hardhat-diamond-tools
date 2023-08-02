use params::hardhat_task_parameter_derive_impl;
use syn::DeriveInput;

mod params;

/// Derive macro for the `TaskParameter` trait.
///
/// Should do from this:
///
/// ```rust
/// use hardhat_bindings_macro::TaskParameter;
///
/// #[derive(TaskParameter, Default)]
/// pub struct TaskArgs {
///     pub name: String,
///     pub flag: bool,
///     pub optional_description: Option<String>,
/// }
/// ```
///
/// To this:
///
/// ```rust
/// #[derive(Default)]
/// pub struct TaskArgs {
///    /// The name for the task
///    pub name: String,
///    /// Some flag
///    pub flag: bool,
///    /// An optional description param
///    pub optional_description: Option<String>,
/// }
///
/// use wasm_bindgen::prelude::JsValue;
///
/// impl ::hardhat_bindings::params::TaskParameter for TaskArgs {
///     fn add_params_to_task(
///         task: ::hardhat_bindings::bindings::config::ConfigurableTaskDefinition,
///     ) -> ::hardhat_bindings::bindings::config::ConfigurableTaskDefinition {
///         let default = Self::default();
///
///         task.add_param("name", "The name for the task", JsValue::from(default.name), false);
///         task.add_param("flag", "Some flag", JsValue::from(default.flag), false);
///         task.add_param("optional_description", "An optional description param", JsValue::from(default.optional_description), true);
///
///         task
///     }
/// }
/// ```
#[proc_macro_derive(TaskParameter)]
pub fn hardhat_task_parameter_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as DeriveInput);

    match hardhat_task_parameter_derive_impl(item) {
        Ok(value) => value,
        Err(value) => value.into_compile_error(),
    }
    .into()
}
