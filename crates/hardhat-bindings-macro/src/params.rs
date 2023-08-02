use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, DeriveInput, Error, FieldsNamed, Ident, TypePath};

pub(crate) fn hardhat_task_parameter_derive_impl(
    DeriveInput { ident, data, .. }: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    match data {
        syn::Data::Struct(struct_type) => gen_task_parameter_for_struct(struct_type, ident),
        _ => Err(Error::new_spanned(
            ident,
            "Only structs are supported for TaskParameter",
        )),
    }
}

fn gen_task_parameter_for_struct(
    struct_type: DataStruct,
    ident: Ident,
) -> Result<TokenStream, syn::Error> {
    match struct_type.fields {
        syn::Fields::Named(fields) => gen_task_parameter_for_named_fields(fields, ident),
        _ => {
            return Err(syn::Error::new_spanned(
                struct_type.struct_token,
                "Only named fields are supported",
            ))
        }
    }
}

fn gen_task_parameter_for_named_fields(
    fields: FieldsNamed,
    ident: Ident,
) -> Result<TokenStream, syn::Error> {
    let calls = fields
        .named
        .iter()
        .map(gen_task_call_for_field)
        .collect::<Result<Vec<_>, syn::Error>>()?;

    Ok(quote! {
        #[automatically_derived]
        impl ::hardhat_bindings::params::TaskParameter for #ident {
            fn add_params_to_task(
                task: ::hardhat_bindings::bindings::config::ConfigurableTaskDefinition,
            ) -> ::hardhat_bindings::bindings::config::ConfigurableTaskDefinition {
                let default = Self::default();

                #(#calls)*

                task
            }
        }
    }
    .into())
}

fn gen_task_call_for_field(field: &syn::Field) -> Result<TokenStream, syn::Error> {
    let name = field.ident.as_ref().unwrap();
    let name_str = name.to_string().from_case(Case::Snake).to_case(Case::Camel);
    let ty = &field.ty;

    let lines = extract_doc(field);

    let doc = format_doc_comment(&lines);

    let ty = extract_field_param_type(ty)?;

    Ok(match ty {
        OptionalParamType::Option(VariadicParamType::Plain(_)) => quote! {
            task.add_optional_param(
                #name_str,
                #doc,
                ::wasm_bindgen::JsValue::from(default.#name),
            );
        },
        OptionalParamType::Option(VariadicParamType::Vec(_)) => quote! {
            {
                let mut __default = if let Some(__values) = default.#name {
                    let mut __array = ::js_sys::Array::new();

                    for value in __values {
                        __array.push(&::wasm_bindgen::JsValue::from(value));
                    }

                    __array.into()
                } else {
                    ::js_sys::Array::new().into()
                };

                task.add_optional_variadic_positional_param(
                    #name_str,
                    #doc,
                    __default,
                );
            };
        },
        OptionalParamType::Plain(_) => quote! {
            task.add_param(
                #name_str,
                #doc,
                ::wasm_bindgen::JsValue::from(default.#name.clone()),
            );
        },
        OptionalParamType::Vec(_) => quote! {
            {
                let mut __array = ::js_sys::Array::new();

                for value in default.#name {
                    __array.push(&::wasm_bindgen::JsValue::from(value));
                }

                task.add_variadic_positional_param(
                    #name_str,
                    #doc,
                    __array.into(),
                );
            };
        },
        OptionalParamType::Flag => quote! {
            task.add_flag(
                #name_str,
                #doc,
                default.#name,
            );
        },
    }
    .into())
}

pub enum VariadicParamType {
    Vec(syn::Type),
    Plain(syn::Type),
}

pub enum OptionalParamType {
    Option(VariadicParamType),
    Vec(syn::Type),
    Plain(syn::Type),
    Flag,
}

fn extract_field_param_type(ty: &syn::Type) -> Result<OptionalParamType, syn::Error> {
    let syn::Type::Path(TypePath { path, .. }) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "Only path fields are supported",
        ));
    };

    let segment = path.segments.last().unwrap();
    let ident = &segment.ident;

    match ident.into_token_stream().to_string().as_str() {
        "Option" => {
            let ty = extract_single_generic_arg(segment)?;

            let syn::Type::Path(TypePath { path, .. }) = ty else {
                return Err(syn::Error::new_spanned(
                    ty,
                    "Only path fields are supported",
                ));
            };

            let segment = path.segments.last().unwrap();
            let ident = &segment.ident;

            match ident.into_token_stream().to_string().as_str() {
                "Vec" => {
                    let ty = extract_single_generic_arg(segment)?;

                    Ok(OptionalParamType::Option(VariadicParamType::Vec(
                        ty.clone(),
                    )))
                }
                _ => Ok(OptionalParamType::Option(VariadicParamType::Plain(
                    ty.clone(),
                ))),
            }
        }
        "Vec" => {
            let ty = extract_single_generic_arg(segment)?;

            Ok(OptionalParamType::Vec(ty.clone()))
        }
        _ => {
            if ident.into_token_stream().to_string().as_str() == "bool" {
                Ok(OptionalParamType::Flag)
            } else {
                Ok(OptionalParamType::Plain(ty.clone()))
            }
        }
    }
}

fn extract_single_generic_arg(segment: &syn::PathSegment) -> Result<&syn::Type, syn::Error> {
    let args = &segment.arguments;

    let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        args,
        ..
    }) = args else {
        return Err(syn::Error::new_spanned(
            args,
            "The `Option` type must have a generic argument",
        ));
    };

    let Some(arg) = args.first() else {
        return Err(syn::Error::new_spanned(
            args,
            "The `Option` type must have at least one generic argument",
        ));
    };

    let syn::GenericArgument::Type(ty) = arg else {
        return Err(syn::Error::new_spanned(
            arg,
            "The `Option` type must have a type generic argument",
        ));
    };

    Ok(ty)
}

fn extract_doc(field: &syn::Field) -> Vec<String> {
    let mut lines: Vec<_> = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            // non #[doc = "..."] attributes are not our concern
            // we leave them for rustc to handle
            match &attr.meta {
                syn::Meta::NameValue(syn::MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }),
                    ..
                }) => Some(s.value()),
                _ => None,
            }
        })
        .skip_while(|s| is_blank(s))
        .flat_map(|s| {
            let lines = s
                .split('\n')
                .map(|s| {
                    // remove one leading space no matter what
                    let s = s.strip_prefix(' ').unwrap_or(s);
                    s.to_owned()
                })
                .collect::<Vec<_>>();
            lines
        })
        .collect();

    while let Some(true) = lines.last().map(|s| is_blank(s)) {
        lines.pop();
    }

    lines
}

fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn format_doc_comment(lines: &[String]) -> String {
    let mut doc = String::new();

    for line in lines {
        doc.push_str(line);
        doc.push('\n');
    }

    doc
}
