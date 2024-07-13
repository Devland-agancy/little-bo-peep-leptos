#![allow(unused_imports)]
#[macro_use]
extern crate proc_macro;
extern crate nom;

use leptos::error::Error;
use proc_macro::Ident;
use proc_macro::TokenStream;
use quote::quote;
use serde_json;
use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use syn::{parse_macro_input, LitStr};

#[derive(Clone, Copy)]
enum ArticleType {
    CHAPTER,
    BOOTCAMP,
}

impl ArticleType {
    fn from_str(_str: &str) -> Self {
        match _str {
            "chapters" => Self::CHAPTER,
            "bootcamps" => Self::BOOTCAMP,
            _ => Self::CHAPTER,
        }
    }
    fn to_str(&self) -> String {
        match self {
            ArticleType::CHAPTER => "chapter".to_string(),
            ArticleType::BOOTCAMP => "bootcamp".to_string(),
        }
    }
    fn to_upper_str(&self) -> String {
        match self {
            ArticleType::CHAPTER => "Chapter".to_string(),
            ArticleType::BOOTCAMP => "Bootcamp".to_string(),
        }
    }
    fn to_abrv(&self) -> String {
        match self {
            ArticleType::CHAPTER => "ch".to_string(),
            ArticleType::BOOTCAMP => "bt".to_string(),
        }
    }
}

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

fn get_sorted_articles(article_type: ArticleType) -> Vec<(u8, PathBuf)> {
    let entries = read_content_dir();
    let mut articles = Vec::<(u8, PathBuf)>::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let path_str = path.to_str().unwrap();

        if metadata.is_dir() && ends_with_article_number(&path, &article_type) {
            let last_char = path_str.chars().last().unwrap();
            let article_number = last_char.to_digit(10).unwrap() as u8;
            articles.push((article_number, path));
        };
    }
    articles.sort_by(|a, b| a.0.cmp(&b.0));
    articles
}

fn ends_with_article_number(path: &PathBuf, article_type: &ArticleType) -> bool {
    let path_str = path.to_str().unwrap_or("");

    let suffix = article_type.to_str();

    if path_str.ends_with(suffix.as_str()) {
        return false; // Ends exactly with " chapter ", which is not valid
    }

    if let Some(pos) = path_str.rfind(suffix.as_str()) {
        let remaining = &path_str[pos + suffix.len()..];

        return remaining.chars().all(|c| c.is_digit(10));
    }
    false
}

struct Input {
    article_type: LitStr,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let article_type: LitStr = input.parse()?;
        Ok(Input { article_type })
    }
}

#[proc_macro]
pub fn render_article_routes(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let article_type: LitStr = input_tokens.article_type;
    let article_type: ArticleType = ArticleType::from_str(article_type.value().as_str());
    let mut routes = String::new();
    let articles = get_sorted_articles(article_type);

    for (i, _) in articles {
        routes.push_str(&format!(
            r#"<Route path="/article/{}_{i}" view=crate::page::article::{}{i}View />"#,
            article_type.to_abrv(),
            article_type.to_upper_str()
        ));
    }

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
pub fn render_article_modules(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let article_type: LitStr = input_tokens.article_type;
    let article_type = ArticleType::from_str(article_type.value().as_str());
    let article_type_str = article_type.to_str();
    let article_type_upper_str = article_type.to_upper_str();

    let mut modules = String::new();
    let elm_only_for: Option<u8> = None;
    let articles = get_sorted_articles(article_type);
    for (i, path) in articles {
        let (title, mobile_title) = get_article_title(&path);
        modules.push_str(&format!(
            r#"
                #[component]
                pub fn {article_type_upper_str}{i}View(cx: Scope) -> impl IntoView {{
                    view! {{ cx,
                    <ArticleTitle label="{title}" {}/>
                    <Columns>
                        <{article_type_upper_str}{i}Body />
                    </Columns>
                    }}
                }}

                #[component]
                fn {article_type_upper_str}{i}Body(cx: Scope) -> impl IntoView {{
                    {}! {{
                    cx,
                    "file:/src/content/{article_type_str}{i}/{article_type_str}_emu.rs"
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
pub fn render_articles_list(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    // Extract the HTML string
    let article_type: LitStr = input_tokens.article_type;
    let article_type = ArticleType::from_str(article_type.value().as_str());
    let article_type_abrv = article_type.to_abrv();

    let mut list = String::new();
    let articles = get_sorted_articles(article_type);
    for (i, path) in articles {
        let (title, mobile_title) = get_article_title(&path);
        list.push_str(&format!(
            r#"
            <MenuItem label="{title}" on_mobile="{mobile_title}" href="{article_type_abrv}_{i}"/>
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

struct InputMultiString {
    article_type: LitStr,
    content_to_render: LitStr,
}

impl syn::parse::Parse for InputMultiString {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let article_type: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let content_to_render: LitStr = input.parse()?;
        Ok(InputMultiString {
            article_type,
            content_to_render,
        })
    }
}

#[proc_macro]
pub fn render_content_for_article(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as InputMultiString);
    // Extract the HTML string
    let article_type: LitStr = input_tokens.article_type;
    let content_to_render: LitStr = input_tokens.content_to_render;

    let article_type = ArticleType::from_str(article_type.value().as_str());

    let mut res = String::new();
    let articles = get_sorted_articles(article_type);

    if articles.len() > 0 {
        res.push_str(content_to_render.value().as_str());
    }

    res = format!("view! {{ cx, {} }}", res);

    let parsed_code = res.parse::<proc_macro2::TokenStream>().unwrap();
    let output = quote! {
        #parsed_code
    };
    output.into()
}

fn get_article_title(path: &PathBuf) -> (String, String) {
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
