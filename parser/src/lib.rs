#![allow(unused_imports)]
#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_parser::transform::{AutoWrapper, Transformer};
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
    let _cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;

    let mut transformer: Transformer = Transformer::new(
        vec!["img", "SectionDivider", "StarDivider", "InlineImage"],
        vec![
            AutoWrapper {
                tags: vec!["ExerciseQuestion", "Example", "Section", "Solution"],
                wrap_children_with: "Paragraph",
                enable_manual_wrap: true,
            },
            AutoWrapper {
                tags: vec!["Grid"],
                wrap_children_with: "Span",
                enable_manual_wrap: true,
            },
            AutoWrapper {
                tags: vec!["List"],
                wrap_children_with: "Item",
                enable_manual_wrap: true,
            },
        ],
        vec!["Example"],
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
            "List",
            "Grid",
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
            "Item",
        ],
        vec!["Grid", "List"],
    );

    let elm_string = if elm.value().starts_with("file:") {
        let file = format!(
            "{}{}",
            env::current_dir().unwrap().display(),
            &elm.value()[5..]
        );

        match fs::read_to_string(file) {
            Ok(contents) => {
                let mut lines = contents.lines();

                // Skip the first line
                if let Some(_) = lines.next() {
                    let mut output = String::new();

                    // Iterate over the remaining lines, excluding the last line
                    for line in lines.clone().take(lines.count() - 1) {
                        output.push_str(line);
                        output.push('\n');
                    }
                    output
                } else {
                    "File is empty".to_string()
                }
            }
            Err(_) => "File not found".to_string(),
        }
    } else {
        elm.value()
    };

    let mut pre = transformer.pre_process_exercises(&elm_string);
    pre = transformer.pre_process_solutions(pre);
    pre = transformer.auto_increamental_title(pre, "Example", "Example", None, None);
    pre = transformer.auto_increamental_title(
        pre,
        "Exercise",
        "Exercise",
        Some("ExerciseQuestion"),
        Some("Solution"),
    );
    pre = transformer.remove_empty_line_above(
        pre,
        vec!["ImageRight", "ImageLeft"],
        Some(("_attached", "false")),
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
