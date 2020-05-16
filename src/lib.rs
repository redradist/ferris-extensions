use proc_macro::TokenStream;
use std::borrow::Borrow;
use quote::{quote, format_ident};
use syn::{ReturnType, Type, PathArguments, GenericArgument};
use std::ops::Add;
use syn::export::Formatter;
use inflector::Inflector;
use std::fmt::Error;
use proc_macro2::TokenTree::Ident;

#[proc_macro_attribute]
pub fn boxed_async_recursion(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let attr_args = syn::parse_macro_input!(attrs as syn::AttributeArgs);

    let sig = &input.sig;
    if sig.asyncness.is_none() {
        panic!("The async keyword is missing from the function declaration !!");
    }

    let vis = input.vis;
    let name = &input.sig.ident;
    let mut args = Vec::new();
    for arg in &input.sig.inputs {
        args.push(arg);
    }
    let ret = match input.sig.output {
        ReturnType::Default => {
            panic!();
        },
        ReturnType::Type(arrow, box_type) => {
            box_type
        }
    };
    let body = &input.block;
    let attrs = &input.attrs;

    let res_fun = quote! {
        #(#attrs)*
        #vis fn #name (#(#args),*) -> BoxFuture<'static, #ret> {
            async move { #body }.boxed()
        }
    };

    let print_tokens = Into::<TokenStream>::into(res_fun.clone());
    println!("Result Function is {}", print_tokens.to_string());
    res_fun.into()
}


trait ResultMultipleErrors {
    fn get_result_multiple_errors(&self) -> Option<(&Type, Vec<&Type>)>;
}

impl ResultMultipleErrors for Type {
    fn get_result_multiple_errors(&self) -> Option<(&Type, Vec<&Type>)> {
        return match self {
            Type::Path(type_path) => {
                if let Some(seg) = type_path.path.segments.last() {
                    let result_type = seg.ident.to_string();
                    if result_type.as_str() == "Result" {
                        return match &seg.arguments {
                            PathArguments::AngleBracketed(args) => {
                                let result: Option<&Type> = match &args.args[0]
                                {
                                    GenericArgument::Type(ty) => {
                                        Some(ty)
                                    },
                                    _ => {
                                        None
                                    }
                                };
                                let errors: Option<Vec<&Type>> = match &args.args[1]
                                {
                                    GenericArgument::Type(ty) => {
                                        match ty {
                                            Type::Tuple(type_tuple) => {
                                                let mut errors = Vec::new();
                                                for sdasd in &type_tuple.elems {
                                                    errors.push(sdasd);
                                                }
                                                Some(errors)
                                            },
                                            _ => {
                                                None
                                            }
                                        }
                                    },
                                    _ => {
                                        None
                                    }
                                };
                                if let Some(errors) = errors {
                                    Some((result.unwrap(), errors))
                                } else {
                                    None
                                }
                            },
                            _ => {
                                None
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => {
                None
            }
        };
    }
}

#[proc_macro_attribute]
pub fn multiple_result_errors(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let attr_args = syn::parse_macro_input!(attrs as syn::AttributeArgs);

    let sig = &input.sig;
    let asyn = &sig.asyncness;
    let cnst = sig.constness;
    let vis = input.vis;
    let name = &sig.ident;
    let mut args = Vec::new();
    for arg in &sig.inputs {
        args.push(arg);
    }
    let ret = match &sig.output {
        ReturnType::Default => {
            panic!("Used only for Result<?,?>");
        },
        ReturnType::Type(arrow, box_type) => {
            box_type
        }
    };
    let body = &input.block;
    let attrs = &input.attrs;
    let res_fun = if let Some((res, errors)) = ret.get_result_multiple_errors() {
        let gen_error = format!("{}", format_ident!("{}_ResultErrors", name)).to_camel_case();
        let mut v: Vec<char> = gen_error.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let gen_error: String = v.into_iter().collect();
        let gen_error = format_ident!("{}", gen_error);
        quote! {
            #vis enum #gen_error {
        #(
            #errors(#errors)
        ),*
            }

        #(
            impl From<#errors> for #gen_error {
                fn from(err: #errors) -> Self {
                    #gen_error::#errors(err)
                }
            }
        )*

            #(#attrs)*
            #vis #asyn #cnst fn #name (#(#args),*) -> Result<#res, #gen_error> {
                #body
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #asyn #cnst fn #name (#(#args),*) -> #ret {
                #body
            }
        }
    };

    let print_tokens = Into::<TokenStream>::into(res_fun.clone());
    println!("Result Function is {}", print_tokens.to_string());
    res_fun.into()
}
