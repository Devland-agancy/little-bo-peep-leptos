#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate proc_macro;
extern crate nom;

use elm_to_view::parse;
use leptos::error::Error;
use proc_macro::Ident;
use proc_macro::TokenStream;
use quote::quote;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::ReadDir;
use std::io::Write;
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
    fn to_upper_char(&self) -> String {
        match self {
            ArticleType::CHAPTER => "C".to_string(),
            ArticleType::BOOTCAMP => "B".to_string(),
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

fn get_sorted_articles(article_type: ArticleType) -> Vec<(usize, PathBuf)> {
    let entries = read_content_dir();
    let mut articles = Vec::<(usize, PathBuf)>::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let path_str = path.to_str().unwrap();
        if entry.file_name().to_str().unwrap().starts_with("#") {
            continue;
        }
        if metadata.is_dir() && ends_with_article_number(&path, &article_type) {
            let last_char = path_str.chars().last().unwrap();
            let article_number = last_char.to_digit(10).unwrap() as usize;
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
    let article_types: LitStr = input_tokens.article_type;
    let article_types = article_types.value();
    let article_types = article_types.split(" ");
    let mut routes = String::new();
    routes.push_str(
        r#"
    <Routes>
        <Route path="" view=crate::page::home::View/>"#,
    );

    for article_type_str in article_types {
        let article_type: ArticleType = ArticleType::from_str(article_type_str);
        let articles = get_sorted_articles(article_type);

        for (i, (article_i, _)) in articles.iter().enumerate() {
            let number = i + 1;

            routes.push_str(&format!(
                r#"<Route path="/article/{}_{number}" view=crate::page::article::{}{article_i}View />"#,
                article_type.to_abrv(),
                article_type.to_upper_str()
            ));
        }
    }
    routes.push_str(r#"</Routes>"#);

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
pub fn render_mods(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    let article_types: LitStr = input_tokens.article_type;
    let article_types = article_types.value();
    let article_types: std::str::Split<'_, &str> = article_types.split(" ");

    let mut modules = String::new();
    for article_type_str in article_types {
        let article_type: ArticleType = ArticleType::from_str(article_type_str);
        let article_type_str = article_type.to_str();

        let articles = get_sorted_articles(article_type);
        for (i, _) in articles {
            modules.push_str(&format!("pub mod {article_type_str}{i};"))
        }
    }
    let parsed_code = modules.parse::<proc_macro2::TokenStream>().unwrap();
    parsed_code.into()
}

#[proc_macro]
pub fn render_article_modules(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as Input);
    let article_types: LitStr = input_tokens.article_type;
    let article_types = article_types.value();
    let article_types: std::str::Split<'_, &str> = article_types.split(" ");
    let mut modules = String::new();
    let show_only: Option<usize> = None;

    let types = article_types
        .clone()
        .into_iter()
        .map(|at| {
            let article_type: ArticleType = ArticleType::from_str(at);
            article_type.to_str().to_string()
        })
        .collect();

    let book = parse(&types, show_only);

    for article_type_str in article_types {
        let article_type: ArticleType = ArticleType::from_str(article_type_str);
        let article_type_upper_str = article_type.to_upper_str();
        let article_type_str = article_type.to_str();

        let articles = get_sorted_articles(article_type);

        for (i, (article_i, path)) in articles.iter().enumerate() {
            let number = i + 1;
            let (title, mobile_title) = get_article_title(&path);
            let mut content = &String::new();
            if show_only.is_none() || show_only.is_some_and(|s| s == i) {
                content = book.get(&format!("{article_type_str}{article_i}")).unwrap();
            }
            modules.push_str(&format!(
                r#"
                #[component]
                pub fn {article_type_upper_str}{article_i}View(cx: Scope) -> impl IntoView {{
                    view! {{ cx,
                    <ArticleTitle label="{article_type_upper_str} {number}: {title}" {}/>
                    <Columns>
                        <{article_type_upper_str}{article_i}Body />
                    </Columns>
                    }}
                }}

                #[component]
                fn {article_type_upper_str}{article_i}Body(cx: Scope) -> impl IntoView {{
                    view! {{
                    cx,
                    {}
                    }}
                }}

                #[component]
                pub fn {article_type_upper_str}{article_i}(cx: Scope, children: Children, title: &'static str) -> impl IntoView {{
                    view! {{ cx,
                    {{children(cx)}}
                    }}
                }}

            "#,
                if mobile_title.is_empty() {
                    "".to_string()
                } else {
                    format!(r#"mobile_title="{mobile_title}""#)
                },
                if show_only.is_some_and(|s| s != i) {
                    "".to_string()
                } else {
                    content.to_string()
                }
            ));
        }
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
    //let article_type_upper = article_type.to_upper_char();

    let mut list = String::new();
    let articles = get_sorted_articles(article_type);
    for (i, (_, path)) in articles.iter().enumerate() {
        let (title, mobile_title) = get_article_title(&path);
        let number = i + 1;
        list.push_str(&format!(
            r#"
            <MenuItem article_type="{number}" label="{title}" on_mobile="{mobile_title}" href="{article_type_abrv}_{number}"/>
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

struct EnvMacro {
    dev_content: LitStr,
    prod_content: LitStr,
}

impl syn::parse::Parse for EnvMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dev_content: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let prod_content: LitStr = input.parse()?;
        Ok(EnvMacro {
            dev_content,
            prod_content,
        })
    }
}

#[proc_macro]
pub fn render_based_on_env(input: TokenStream) -> TokenStream {
    let input_tokens = parse_macro_input!(input as EnvMacro);
    // Extract the HTML string
    let dev_content: LitStr = input_tokens.dev_content;
    let prod_content: LitStr = input_tokens.prod_content;
    if get_env_var("ENV") == "development" {
        let parsed_code = dev_content
            .value()
            .parse::<proc_macro2::TokenStream>()
            .unwrap();
        let output = quote! {
            view!{
                cx,
                #parsed_code
            }
        };
        output.into()
    } else {
        let parsed_code = prod_content
            .value()
            .parse::<proc_macro2::TokenStream>()
            .unwrap();
        let output = quote! {
            view!{
                cx,
                #parsed_code
            }
        };
        output.into()
    }
}

fn get_env_var(key: &str) -> String {
    if let Ok(value) = env::var(key) {
        return value;
    }

    let path = env::current_dir();

    let path_string = format!("{}/.env", path.unwrap().display());
    let env_file = read_to_string(&path_string);
    if env_file.is_err() {
        panic!("env not found {}", path_string);
    }
    let env_str = env_file.unwrap();
    let lines = env_str.lines();
    let mut env_value = String::new();

    for line in lines {
        let l = line.split_once("=");
        if l.is_none() {
            continue;
        }
        let (_key, value) = l.unwrap();
        if _key == key {
            env_value = value.to_string();
            break;
        }
    }
    env_value
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
        if file_name == "__parent.emu" {
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
            return (title, mobile_title);
        }
    }

    (title, mobile_title)
}

fn write_to_file(file_path: &str, contents: &str) {
    let mut json_file: File = match File::create(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error creating file: {}", error);
            return;
        }
    };

    match json_file.write_all(contents.as_bytes()) {
        Ok(_) => (),
        Err(error) => println!("Error writing to {file_path}: {error}"),
    }
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
