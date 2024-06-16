#![allow(unused_imports)]
#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_parser::desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use elm_parser::emitter::Emitter;
use elm_parser::parser::Parser;
use elm_parser::parser_helpers::DataCell;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use serde_json;
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

fn get_root(json: &str) -> DataCell {
    return serde_json::from_str(json).unwrap();
}

#[proc_macro]
pub fn elm(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let _cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;

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

    let mut json = Parser::new();
    let json_tree = json.export_json(&elm_string, None, false);

    let mut desugarer: Desugarer = Desugarer::new(json_tree.as_str(), json.id);
    desugarer = desugarer
        .pre_process_exercises()
        .pre_process_solutions()
        .auto_increamental_title("Exercise", "Exercise")
        .auto_increamental_title("Example", "Example")
        .wrap_block_delimited("InnerParagraph")
        .wrap_children(
            vec!["Section", "Solution", "Example", "Exercise"],
            "Paragraph",
            &Some(vec![
                IgnoreOptions {
                    element: "InlineImage",
                    attach_to: AttachToEnum::BOTH,
                },
                IgnoreOptions {
                    element: "ImageRight",
                    attach_to: AttachToEnum::BEFORE,
                },
                IgnoreOptions {
                    element: "ImageLeft",
                    attach_to: AttachToEnum::BEFORE,
                },
                IgnoreOptions {
                    element: "del",
                    attach_to: AttachToEnum::BOTH,
                },
            ]),
        )
        .wrap_children(vec!["Grid"], "Span", &None)
        .wrap_children(vec!["List", "del"], "Item", &None)
        .add_indent(&ParagraphIndentOptions {
            tags_before_non_indents: vec![
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
                "del",
            ],
            tags_with_non_indent_first_child: vec![
                "Section",
                "Example",
                "Solution",
                "Table",
                "td",
                "ImageLink",
                "Paragraph",
                "ExerciseQuestion",
                "Exercise",
                "Item",
            ],
        })
        .add_attribute(vec!["Solution", "Example"], ("no_padding", "true"));

    let json_value: DataCell = serde_json::from_str(&desugarer.json).unwrap();

    let emitter: Emitter =
        Emitter::new(vec!["img", "SectionDivider", "InlineImage", "StarDivider"]);
    let leptos_code = emitter.emit_json(&json_value);

    let parsed_code = leptos_code.parse::<proc_macro2::TokenStream>().unwrap();

    let output = quote! {
        view! {
            cx, #parsed_code
        }
    };
    output.into()
}
