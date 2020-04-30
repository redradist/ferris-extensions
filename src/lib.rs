use proc_macro::TokenStream;
use std::borrow::Borrow;
use quote::quote;
use syn::{ReturnType, Type};

#[proc_macro_attribute]
pub fn async_recursive(attrs: TokenStream, item: TokenStream) -> TokenStream {
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

    let result = quote! {
        #(#attrs)*
        #vis fn #name (#(#args),*) -> BoxFuture<'static, #ret> {
            async move { #body }.boxed()
        }
    };

    let print_tokens = Into::<TokenStream>::into(result.clone());
    println!("gen fun is {}", print_tokens.to_string());
    result.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
