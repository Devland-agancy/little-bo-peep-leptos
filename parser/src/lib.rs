#![allow(unused_imports)]
#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_parser::counter::counter_commands::CounterCommand;
use elm_parser::counter::counters::Counters;
use elm_parser::desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use elm_parser::emitter::Emitter;
use elm_parser::parser::Parser;
use elm_parser::parser_helpers::DataCell;

use leptos::error::Error;
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

#[proc_macro]
pub fn elm(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let _cx = input_tokens.cx;
    let elm: LitStr = input_tokens.elm;

    let elm_string = if elm.value().starts_with("file:") {
        let mut path = env::current_dir().unwrap();
        path.pop();
        let _path_string = format!("{}/src/content", path.display());
        //let _res = get_content(path_string.as_str());
        // if let Ok(content) = res {
        //     content
        // } else {
        //     panic!("Error");
        // }

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

    let mut counters = Counters::new();

    let mut json = Parser::new(&mut counters);
    let json_tree = json.export_json(&elm_string, None, false);

    //let mut counter_command = CounterCommand::new(&mut counters, &json_tree);
    //let mut json: DataCell = serde_json::from_str(&json_tree).unwrap();
    //let json_tree = counter_command.run(&mut json);

    let mut desugarer: Desugarer = Desugarer::new(json_tree.as_str(), json.id);
    desugarer = desugarer
        .pre_process_exercises()
        .add_increamental_attr(vec![("Solution", "solution_number")])
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
                IgnoreOptions {
                    element: "Space",
                    attach_to: AttachToEnum::BEFORE,
                },
            ]),
        )
        .wrap_children(vec!["Grid"], "Span", &None)
        .wrap_children(vec!["List"], "Item", &None)
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
                "List",
                "Grid",
                "DisplayCentered",
            ],
            tags_with_non_indent_first_child: vec![
                "Section",
                "Example",
                "Solution",
                "Table",
                "td",
                "ImageLink",
                "ExerciseQuestion",
                "Exercise",
                "Item",
            ],
        })
        .add_attribute(vec!["Solution", "Example"], ("no_padding", "true"))
        .auto_convert_to_float(vec!["line", "padding_left"]);

    let json_value: DataCell = serde_json::from_str(&desugarer.json).unwrap();

    let mut emitter: Emitter = Emitter::new(
        &json_value,
        vec!["img", "col", "SectionDivider", "InlineImage", "StarDivider"],
    );
    let leptos_code = emitter.emit_json(&json_value);

    let parsed_code = leptos_code.parse::<proc_macro2::TokenStream>().unwrap();

    let output = quote! {
        view! {
            cx, #parsed_code
        }
    };
    output.into()
}

fn get_content(path_str: &str) -> Result<String, Error> {
    //read directory files

    let entries = fs::read_dir(path_str).unwrap();
    let mut chapter = String::new();
    let mut book = String::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if entry.file_name() == "parent_emu.rs" {
            let mut file_content = fs::read_to_string(path).unwrap();
            file_content = remove_comment_symbols(&file_content);
            book.push_str(&file_content);
        } else if entry.file_name() == "chapter_emu.rs" {
            let mut file_content = fs::read_to_string(path).unwrap();
            file_content = remove_comment_symbols(&file_content);
            let with_indent = add_indent(&file_content);
            chapter.push_str(&with_indent);
        } else if metadata.is_dir() {
            if let Ok(nested_content) = get_content(&path.to_str().unwrap()) {
                let with_indent = add_indent(&nested_content);
                book.push_str(&with_indent);
            }
        };
    }
    book.push_str(&chapter);
    Ok(book)
}

fn remove_comment_symbols(content: &str) -> String {
    let mut output = String::new();
    let mut lines = content.lines();

    // Skip the first line
    if let Some(_) = lines.next() {
        // Iterate over the remaining lines, excluding the last line
        for line in lines.clone().take(lines.count() - 1) {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}

fn add_indent(content: &str) -> String {
    let mut indented_first_line = String::new();
    let lines = content.lines();

    // add indent
    for line in lines.clone().take(lines.count()) {
        indented_first_line.push_str(&format!("    {}", line));
        indented_first_line.push('\n');
    }
    indented_first_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content() {
        let mut path = env::current_dir().unwrap();
        path.pop();
        let path_string = format!("{}/src/content", path.display());
        let res = get_content(path_string.as_str());
        if let Ok(content) = res {
            panic!("content {}", content);
        } else {
            panic!("Error");
        }
    }
}
