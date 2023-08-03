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
///     /// Small doc
///     pub flag: bool,
///     /// Big doc
///     ///
///     /// With multiple lines
///     pub flag1: bool,
///     /// Big doc
///     ///
///     /// With multiple lines
///     ///
///     /// And even more
///     pub name: String,
///     pub optional: Option<String>,
///     pub variadic: Vec<String>,
///     pub optional_variadic: Option<Vec<String>>,
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
///         task.add_flag("flag", "Small doc\n", default.flag);
///         task.add_flag("flag1", "Big doc\n\nWith multiple lines\n", default.flag1);
///         task.add_param(
///             "name",
///             "Big doc\n\nWith multiple lines\n\nAnd even more\n",
///             ::wasm_bindgen::JsValue::from(default.name.clone()),
///         );
///         task.add_optional_param(
///             "optional",
///             "",
///             ::wasm_bindgen::JsValue::from(default.optional),
///         );
///         {
///             let mut __array = ::js_sys::Array::new();
///             for value in default.variadic {
///                 __array.push(&::wasm_bindgen::JsValue::from(value));
///             }
///             task.add_variadic_positional_param("variadic", "", __array.into());
///         };
///         {
///             let mut __default = if let Some(__values) = default.optional_variadic {
///                 let mut __array = ::js_sys::Array::new();
///                 for value in __values {
///                     __array.push(&::wasm_bindgen::JsValue::from(value));
///                 }
///                 __array.into()
///             } else {
///                 ::js_sys::Array::new().into()
///             };
///             task.add_optional_variadic_positional_param(
///                 "optionalVariadic",
///                 "",
///                 __default,
///             );
///         };
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
