#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_parser::transform::Transformer;
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
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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

    let mut transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider"],
        vec!["Section", "Example", "Solution", "ExerciseQuestion"],
        vec!["Example"],
        "Paragraph",
        vec![
            "Image",
            "DisplayImage",
            "Pause",
            "StarDivider",
            "MathBlock",
            "Table",
            "SectionDivider",
            "Example",
            "InlineImage",
        ],
        vec![
            "Section",
            "Example",
            "Solution",
            "Table",
            "td",
            "ImageLink",
            "Paragraph",
            "ExerciseQuestion",
        ],
    );

    let mut elm_string = if elm.value().starts_with("file:") {
        let file = format!(
            "{}{}",
            env::current_dir().unwrap().display(),
            &elm.value()[5..]
        );

        match fs::read_to_string(file) {
            Ok(contents) => contents.to_string(),
            Err(_) => "File not found".to_string(),
        }
    } else {
        elm.value()
    };

    let mut pre = transformer.pre_process_exercises(elm_string);
    pre = transformer.auto_increamental_title(pre, "Example", "Example", None, None);
    pre = transformer.auto_increamental_title(
        pre,
        "Exercise",
        "Exercise",
        Some("ExerciseQuestion"),
        Some("Solution"),
    );

    let leptos_code = transformer.transform(pre, 0);
    let parsed_code = leptos_code.parse::<proc_macro2::TokenStream>().unwrap();

    let output = quote! {
        view! {
            cx, #parsed_code
        }
    };
    output.into()
}
