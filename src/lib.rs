use proc_macro::TokenStream;
use std::borrow::Borrow;
use quote::quote;
use syn::{ReturnType, Type};

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
