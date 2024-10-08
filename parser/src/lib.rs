#![allow(unused_imports)]

use elm_parser::counter::counter_commands::CounterCommand;
use elm_parser::counter::counters::Counters;
use elm_parser::datacell::Datacell::DataCell;
use elm_parser::desugarer::{AttachToEnum, Desugarer, IgnoreOptions, ParagraphIndentOptions};
use elm_parser::emitter::Emitter;
use elm_parser::parser::{Parser, ParserError};

use leptos::error::Error;
use quote::quote;
use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::str::Lines;

fn write_to_file(file_path: &str, contents: &str) {
    let mut json_file: File = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match json_file.write_all(contents.as_bytes()) {
        Ok(_) => println!("file written"),
        Err(error) => println!("Error writing to {file_path}: {error}"),
    }
}

pub fn parse(article_types: &Vec<String>, show_only: Option<usize>) -> HashMap<String, String> {
    let path = env::current_dir().unwrap();
    let path_string = format!("{}/src/content", path.display());
    let mut files_with_lines_number = vec![];
    let res = get_content(
        path_string.as_str(),
        article_types,
        show_only,
        &mut files_with_lines_number,
        None,
        0,
    );
    if res.is_err() {
        panic!("Error");
    }

    let elm_string = res.unwrap();
    write_to_file(&format!("{}/src/res.emu", path.display()), &elm_string);

    let mut json = Parser::new();
    let parsed_json_string = json.export_json(&elm_string, None, false);

    if let Err(err) = parsed_json_string {
        match err {
            ParserError::ExtraSpacesError(line) | ParserError::None4xSpacesError(line) => {
                //since our files are merged we need to get the file and relevant line
                let mut temp = 0;
                let mut i = 0;
                let mut file_line_error = line;
                while temp <= line {
                    temp += files_with_lines_number[i].1;
                    if temp <= line {
                        file_line_error -= files_with_lines_number[i].1;
                    }
                    i += 1;
                }
                let file_with_error = &files_with_lines_number[i - 1].0;
                file_line_error = file_line_error;
                panic!(
                    "\nError on File: {file_with_error} \nOn line: {file_line_error} \nMessage: {} ",
                    err.to_string_without_line()
                );
            }
        }
    }
    let parsed_json_string = parsed_json_string.unwrap();

    let mut desugarer: Desugarer = Desugarer::new(parsed_json_string.as_str(), json.id);
    let types: Vec<String> = article_types
        .iter()
        .map(|x| (x.as_str()[0..1].to_uppercase() + &x[1..]).to_string())
        .collect();

    let wrap_ignored_elements = Some(vec![
        IgnoreOptions {
            element: "InlineImage",
            attach_to: AttachToEnum::BOTH,
        },
        IgnoreOptions {
            element: "br",
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
        IgnoreOptions {
            element: "Pause",
            attach_to: AttachToEnum::NONE,
        },
    ]);

    desugarer = desugarer
        .pre_process_exercises()
        .add_increamental_attr(vec![("Solution", "solution_number")], &types)
        .auto_increamental_title("Exercise", "Exercise", &types)
        .auto_increamental_title("Example", "Example", &types)
        .wrap_block_delimited("InnerParagraph")
        .wrap_children(vec!["Section"], "Paragraph", &wrap_ignored_elements)
        .wrap_children(
            vec!["Solution", "Example", "Exercise"],
            "InnerParagraph",
            &wrap_ignored_elements,
        )
        .wrap_children(vec!["Grid"], "Span", &None)
        .wrap_children(vec!["List"], "Item", &None)
        .add_indent(&vec!["Paragraph", "InnerParagraph"])
        .add_attribute(vec!["Solution", "Example"], ("no_padding", "true"))
        .auto_convert_to_float(vec!["line", "padding_left"]);

    let mut desugarer_json: DataCell = serde_json::from_str(&desugarer.json).unwrap();

    // counter
    let mut counters = Counters::new();
    counters.get_counters_from_json(&desugarer_json);
    let mut counter_command = CounterCommand::new(&mut counters, &desugarer.json);
    let json_counter_string = counter_command.run(&mut desugarer_json);

    let json_counter: DataCell = serde_json::from_str(&json_counter_string).unwrap();

    let mut emitter: Emitter = Emitter::new(vec![
        "img",
        "col",
        "SectionDivider",
        "InlineImage",
        "StarDivider",
        "br",
        "Space",
    ]);

    let parsed_map = emitter.split_and_emit(&json_counter, "Book");

    // let mut json_file = File::create("src/res").unwrap();
    // match json_file.write_all(elm_string.as_bytes()) {
    //     Ok(_) => {
    //         println!("Json written to json_output.json successfully");
    //     }
    //     Err(error) => println!("Error writing to json_output.json: {}", error),
    // }

    parsed_map
}

fn get_content(
    path_str: &str,
    article_types: &Vec<String>,
    show_only: Option<usize>,
    files_with_lines_number: &mut Vec<(String, usize)>,
    parent_file_name: Option<&str>,
    recursive_num: usize,
) -> Result<String, Error> {
    //read directory files
    let mut entries: Vec<_> = fs::read_dir(path_str)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    if article_types.len() > 0 {
        entries.sort_by(|a, b| {
            let a_name = a.file_name().into_string().unwrap();
            let b_name = b.file_name().into_string().unwrap();

            let a_index = article_types
                .iter()
                .position(|t| a_name.contains(t))
                .unwrap_or(usize::MAX); // Assign max index if not found
            let b_index = article_types
                .iter()
                .position(|t| b_name.contains(t))
                .unwrap_or(usize::MAX); // Assign max index if not found

            // Sort based on found index, prioritize existing matches
            if a_index == b_index {
                a_name.cmp(&b_name) // If both match or both don't, sort alphabetically
            } else if a_index < b_index {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    } else {
        entries.sort_by(|a, b| {
            let a_name = a.file_name().into_string().unwrap();
            let b_name = b.file_name().into_string().unwrap();
            return a_name.cmp(&b_name);
        })
    }

    let mut book = String::new();

    let mut parent_exists = false;
    entries.iter().any(|en| {
        if en.file_name() == "__parent.emu" {
            parent_exists = true;
            let mut file_content = fs::read_to_string(en.path()).unwrap();
            if let Some(parent_file_name) = parent_file_name {
                let mut new_content = String::new();
                for line in file_content.lines() {
                    if recursive_num == 1 && line.trim().starts_with("|> ") {
                        // append article number to the tag
                        new_content.push_str(line.trim_end());
                        new_content.push_str(&parent_file_name[parent_file_name.len() - 1..]);
                        new_content.push('\n');
                    } else {
                        new_content.push_str(line);
                        new_content.push('\n');
                    }
                }
                file_content = new_content
            }
            book.push_str(&file_content);

            let lines_count = file_content.lines().count();
            files_with_lines_number.push((en.path().to_str().unwrap().to_string(), lines_count));
            return true;
        }
        false
    });

    if !parent_exists {
        panic!("missing __parent file in {path_str}")
    }
    for entry in entries {
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        let file_name = entry.file_name().to_str().unwrap().to_string();
        if file_name.starts_with("#") {
            continue;
        }
        if file_name.ends_with(".emu") && file_name != "__parent.emu" {
            let file_content = fs::read_to_string(&path).unwrap();
            let with_indent = add_indent(&file_content);
            book.push_str("\n");
            book.push_str(&with_indent);

            let lines_count = file_content.lines().count();
            files_with_lines_number.push((path.to_str().unwrap().to_string(), lines_count));
        } else if metadata.is_dir() {
            // book.push_str("&with_indent");

            if let Ok(nested_content) = get_content(
                &path.to_str().unwrap(),
                &vec![],
                show_only,
                files_with_lines_number,
                Some(file_name.as_str()),
                recursive_num + 1,
            ) {
                let with_indent = add_indent(&nested_content);
                book.push_str(&with_indent);
            }
        };
    }

    Ok(book)
}

fn _remove_comment_symbols(content: &str) -> String {
    let mut output = String::new();
    let mut lines = content.lines();

    // Skip the first line
    if !lines.nth(0).unwrap().contains("/*") {
        for line in lines.clone() {
            output.push_str(line);
            output.push('\n');
        }
        return output;
    }

    // Iterate over the remaining lines, excluding the last line
    for line in lines.clone().take(lines.count() - 1) {
        output.push_str(line);
        output.push('\n');
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
