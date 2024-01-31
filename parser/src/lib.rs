#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_parser::transform::transform;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::env;
use std::fs;
use syn::{parse_macro_input, LitStr};
struct Input {
    cx: Ident,
    elm: LitStr,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let cx: syn::Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let elm: LitStr = input.parse()?;
        Ok(Input { cx, elm })
    }
}

#[proc_macro]
pub fn elm(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;
    let leptos_code = if elm.value().starts_with("file:") {
        let file = format!(
            "{}{}",
            env::current_dir().unwrap().display(),
            &elm.value()[5..]
        );

        match fs::read_to_string(file) {
            Ok(contents) => transform(contents.to_string()),
            Err(err) => "File not found".to_string(),
        }
    } else {
        transform(elm.value())
    };

    let parsed_code = leptos_code.parse::<proc_macro2::TokenStream>().unwrap();

    let output = quote! {
        view! {
            cx, #parsed_code
        }
    };
    output.into()
}
