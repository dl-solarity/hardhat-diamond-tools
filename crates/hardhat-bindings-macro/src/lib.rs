use quote::{quote, quote_spanned, spanned::Spanned};
use syn::TypePath;

/// Wrap a function to simplify parsing of the action arguments.
///
/// Should do from this:
///
/// ```rust
/// use hardhat_bindings_macro::hardhat_action;
///
/// #[derive(serde::Deserialize, Default)]
/// pub struct Args {
///    pub contracts: Vec<String>,
/// }
///
/// #[hardhat_action]
/// pub async fn names_action(
///     args: Args,
///     hre: ::hardhat_bindings::HardhatRuntimeEnvironment,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     todo!("Implement me")    
/// }   
/// ```
///
/// To this:
///
/// ```rust
///
/// #[derive(serde::Deserialize, Default)]
/// pub struct Args {
///    pub contracts: Vec<String>,
/// }
///
/// #[::wasm_bindgen::prelude::wasm_bindgen]
/// pub fn names_action(args: ::wasm_bindgen::prelude::JsValue, hre: ::hardhat_bindings::bindings::runtime::HardhatRuntimeEnvironment) -> ::js_sys::Promise {
///     let hre = ::hardhat_bindings::HardhatRuntimeEnvironment::from(hre);
///     let args: Args = match ::serde_wasm_bindgen::from_value(args) {
///         Ok(args) => args,
///         Err(err) => return ::js_sys::Promise::reject(&err.into()),
///     };
///
///     async fn inner(args: Args, hre: ::hardhat_bindings::HardhatRuntimeEnvironment) -> Result<(), Box<dyn std::error::Error>> {
///        todo!("Implement me")
///     }
///    
///     ::wasm_bindgen_futures::future_to_promise(async move {
///         inner(args, hre).await.map_err(|err| ::wasm_bindgen::prelude::JsValue::from(err.to_string()))?;
///
///        Ok(::wasm_bindgen::prelude::JsValue::UNDEFINED)
///     })   
/// }
/// ```
#[proc_macro_attribute]
pub fn hardhat_action(
    _attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    match hardhat_action_impl(item) {
        Ok(value) => value,
        Err(value) => value,
    }
}

/// TODO(Velnbur): refactor this bullshit
fn hardhat_action_impl(
    item: syn::ItemFn,
) -> Result<proc_macro::TokenStream, proc_macro::TokenStream> {
    let name = &item.sig.ident;

    let action_args_type = hardhat_action_get_first_arg(&item)?;
    let block = &item.block;

    let return_type = hardhat_action_get_return_type(&item)?;

    let result_tmpl = quote! {
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn #name(args: ::wasm_bindgen::prelude::JsValue, hre: ::hardhat_bindings::bindings::runtime::HardhatRuntimeEnvironment) -> ::js_sys::Promise {
            let hre = ::hardhat_bindings::HardhatRuntimeEnvironment::from(hre);

            let args: #action_args_type = match serde_wasm_bindgen::from_value(args) {
                Ok(args) => args,
                Err(err) => return ::js_sys::Promise::reject(&err.into()),
            };

            async fn inner(args: #action_args_type, hre: ::hardhat_bindings::HardhatRuntimeEnvironment) -> #return_type {
                #block
            }

            ::wasm_bindgen_futures::future_to_promise(async move {
                inner(args, hre).await.map_err(|err| ::wasm_bindgen::prelude::JsValue::from(err.to_string()))?;

                Ok(::wasm_bindgen::prelude::JsValue::UNDEFINED)
            })
        }
    };

    Ok(result_tmpl.into())
}

fn hardhat_action_get_return_type(
    item: &syn::ItemFn,
) -> Result<&syn::Type, proc_macro::TokenStream> {
    let output = &item.sig.output;

    let return_type = match output {
        syn::ReturnType::Type(_, r#type) => r#type,
        _ => {
            return Err(
                quote_spanned!(output.__span() => compile_error!("Return should not be '()'"))
                    .into(),
            );
        }
    };

    Ok(return_type)
}

fn hardhat_action_get_first_arg(
    item: &syn::ItemFn,
) -> Result<&syn::Ident, proc_macro::TokenStream> {
    let first_arg = item.sig.inputs.first().unwrap();
    let arg_attr = match first_arg {
        syn::FnArg::Typed(path) => &path.ty,
        _ => {
            return Err(quote_spanned!(first_arg.__span() => compile_error!("First argument should not be `self`")).into());
        }
    };

    let arg_type = match arg_attr.as_ref() {
        syn::Type::Path(TypePath { path, .. }) => &path.segments.iter().last().unwrap().ident,
        _ => todo!(),
    };

    Ok(arg_type)
}
