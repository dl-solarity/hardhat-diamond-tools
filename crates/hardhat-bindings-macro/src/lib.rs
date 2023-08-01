use proc_macro::Ident;
use quote::{quote, quote_spanned, spanned::Spanned, ToTokens};
use syn::{parse_macro_input, Path, TypePath};

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
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    let mut no_args = false;
    let attrs_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("no_args") {
            no_args = true;
            return Ok(());
        }

        Err(syn::Error::new_spanned(meta.path, "Unknown attribute"))
    });

    // parse the `attrs` attributes using `attrs_parser`
    syn::parse_macro_input!(attrs with attrs_parser);

    match hardhat_action_impl(item, no_args) {
        Ok(value) => value,
        Err(value) => value,
    }
}

/// TODO(Velnbur): refactor this bullshit
fn hardhat_action_impl(
    item: syn::ItemFn,
    no_args: bool,
) -> Result<proc_macro::TokenStream, proc_macro::TokenStream> {
    let name = &item.sig.ident;
    let block = &item.block;
    let return_type = hardhat_action_get_return_type(&item)?;

    let args_related_block = if !no_args {
        let action_args_type = hardhat_action_get_first_arg(&item)?;
        quote! {
            let _args: #action_args_type = match serde_wasm_bindgen::from_value(_args) {
                Ok(args) => args,
                Err(err) => return ::js_sys::Promise::reject(&err.into()),
            };

            async fn inner(args: #action_args_type, hre: ::hardhat_bindings::HardhatRuntimeEnvironment) -> #return_type {
                #block
            }

            ::wasm_bindgen_futures::future_to_promise(async move {
                inner(_args, hre).await.map_err(|err| ::wasm_bindgen::prelude::JsValue::from(err.to_string()))?;

                Ok(::wasm_bindgen::prelude::JsValue::UNDEFINED)
            })
        }
    } else {
        quote! {
            async fn inner(hre: ::hardhat_bindings::HardhatRuntimeEnvironment) -> #return_type {
                #block
            }

            ::wasm_bindgen_futures::future_to_promise(async move {
                inner(hre).await.map_err(|err| ::wasm_bindgen::prelude::JsValue::from(err.to_string()))?;

                Ok(::wasm_bindgen::prelude::JsValue::UNDEFINED)
            })
        }
    };

    let result_tmpl = quote! {
        #[::wasm_bindgen::prelude::wasm_bindgen]
        pub fn #name(_args: ::wasm_bindgen::prelude::JsValue, hre: ::hardhat_bindings::bindings::runtime::HardhatRuntimeEnvironment) -> ::js_sys::Promise {
            let hre = ::hardhat_bindings::HardhatRuntimeEnvironment::from(hre);

            #args_related_block

        }
    };

    Ok(result_tmpl.into())
}

fn hardhat_action_get_return_type(
    item: &syn::ItemFn,
) -> Result<&Box<syn::Type>, proc_macro::TokenStream> {
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
