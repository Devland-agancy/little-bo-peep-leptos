#![allow(unused_imports)]
#[macro_use]
extern crate proc_macro;
extern crate nom;

use leptos::error::Error;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use serde_json;
use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

fn read_content_dir() -> ReadDir {
    let path = env::current_dir();
    if path.is_err() {
        panic!("Path env error")
    }
    let path_string = format!("{}/src/content", path.unwrap().display());
    let entries = fs::read_dir(path_string);
    if entries.is_err() {
        panic!("Content dir not found")
    }
    entries.unwrap()
}

fn get_sorted_chapters() -> Vec<(u8, PathBuf)> {
    let entries = read_content_dir();
    let mut chapters = Vec::<(u8, PathBuf)>::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();

        if metadata.is_dir() && ends_with_chapter_number(&path) {
            let path_str = path.to_str().unwrap();
            let last_char = path_str.chars().last().unwrap();
            let chapter_number = last_char.to_digit(10).unwrap() as u8;
            chapters.push((chapter_number, path));
        };
    }
    chapters.sort_by(|a, b| a.0.cmp(&b.0));
    chapters
}

fn ends_with_chapter_number(path: &PathBuf) -> bool {
    let path_str = path.to_str().unwrap_or("");

    let suffix = "chapter";

    if path_str.ends_with(suffix) {
        return false; // Ends exactly with " chapter ", which is not valid
    }

    if let Some(pos) = path_str.rfind(suffix) {
        let remaining = &path_str[pos + suffix.len()..];
        return remaining.chars().all(|c| c.is_digit(10));
    }
    false
}

#[proc_macro]
pub fn render_chapter_routes(input: TokenStream) -> TokenStream {
    let mut routes = String::from(
        r#"
    <Routes>
        <Route path="" view=crate::page::home::View/>
    "#,
    );

    let chapters = get_sorted_chapters();
    for (i, _) in chapters {
        routes.push_str(&format!(
            r#"<Route path="/article/ch_{i}" view=crate::page::article::Chapter{i}View />"#
        ));
    }
    routes.push_str(r#"</Routes>"#);
    // routes = format!("view! {{ cx, {} }}", routes);
    let parsed_code = routes.parse::<proc_macro2::TokenStream>().unwrap();
    let output = quote! {
        view! {
            cx,
            #parsed_code
        }
    };
    output.into()
}

#[proc_macro]
pub fn render_chapter_modules(_: TokenStream) -> TokenStream {
    let mut modules = String::new();
    let elm_only_for: Option<u8> = None;
    let chapters = get_sorted_chapters();
    for (i, path) in chapters {
        let (title, mobile_title) = get_chapter_title(&path);
        modules.push_str(&format!(
            r#"
                #[component]
                pub fn Chapter{i}View(cx: Scope) -> impl IntoView {{
                    view! {{ cx,
                    <ArticleTitle label="{title}" {}/>
                    <Columns>
                        <Chapter{i}Body />
                    </Columns>
                    }}
                }}

                #[component]
                fn Chapter{i}Body(cx: Scope) -> impl IntoView {{
                    {}! {{
                    cx,
                    "file:/src/content/chapter{i}/chapter_emu.rs"
                    }}
                }}
            "#,
            if mobile_title.is_empty() {
                "".to_string()
            } else {
                format!(r#"mobile_title="{mobile_title}""#)
            },
            if elm_only_for.is_none() || elm_only_for.is_some_and(|e| e == i) {
                "elm"
            } else {
                "view"
            }
        ));
    }

    let parsed_code = modules.parse::<proc_macro2::TokenStream>().unwrap();

    let output = quote! {
        #parsed_code
    };
    output.into()
}

#[proc_macro]
pub fn render_chapters_list(_: TokenStream) -> TokenStream {
    let mut list = String::new();
    let chapters = get_sorted_chapters();
    for (i, path) in chapters {
        let (title, mobile_title) = get_chapter_title(&path);
        list.push_str(&format!(
            r#"
            <MenuItem label="{title}" on_mobile="{mobile_title}" href="ch_{i}"/>
            "#
        ));
    }

    list = format!("view! {{ cx, {} }}", list);

    let parsed_code = list.parse::<proc_macro2::TokenStream>().unwrap();
    let output = quote! {
        #parsed_code
    };
    output.into()
}

fn get_chapter_title(path: &PathBuf) -> (String, String) {
    let path_str = path.to_str().unwrap_or("");
    let entries = fs::read_dir(path_str);
    if entries.is_err() {
        panic!("Path not found {}", path_str)
    }

    let mut title = String::new();
    let mut mobile_title = String::new();
    for entry in entries.unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap_or("");
        if file_name == "parent_emu.rs" {
            let parent_emu = fs::read_to_string(path);
            if parent_emu.is_err() {
                panic!("Path not found {}", path_str)
            }
            let parent_emu = parent_emu.unwrap();
            let lines = parent_emu.split("\n").collect::<Vec<&str>>();
            for line in lines {
                if line.trim().starts_with("title ") {
                    title += line.trim().split_once(" ").unwrap().1;
                } else if line.trim().starts_with("mobile_title ") {
                    mobile_title += line.trim().split_once(" ").unwrap().1;
                }
            }
            if title.is_empty() {
                panic!("title is empty");
            };
            return (title, mobile_title);
        }
    }
    if title.is_empty() {
        panic!("Could not find title attribute");
    };
    (title, mobile_title)
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
