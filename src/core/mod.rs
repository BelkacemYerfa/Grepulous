use std::{
    env,
    ffi::OsStr,
    fs,
};

use ast::Tokenizer;
use walkdir::WalkDir;

pub mod action;
pub mod ast;

fn search_file(file_name: &str) {
    let workflow = env::current_dir().unwrap();
    let arr = file_name.split(".").collect::<Vec<&str>>();
    let mut current_entries = WalkDir::new(&workflow)
        .into_iter()
        .filter_map(|e| e.ok());
    if arr[0].contains("*") {
        let paths = current_entries
            .filter(|entry| {
                entry.file_type().is_file()
                    &&  entry.path().extension().and_then(OsStr::to_str) == Some(arr[1])
            });
        for entry in paths {
            // * instead of retuning we need to parse it and do the search on it+
            let mut tokenized_output = Tokenizer::new();
            let content = fs::read_to_string(entry.path()).unwrap_or("".to_string());
            tokenized_output.tokenize(content.as_str());

            for token in tokenized_output.tokens {
                println!("the tokens is {}, the id of is {}", token.1, token.0);
            }
        }
    } else {
        let target_path = current_entries
            .find(|entry| {
                entry.file_type().is_file()
                    && entry.path().extension().and_then(OsStr::to_str) == Some(arr[1])
            });
        if let Some(entry) = target_path {
            let mut tokenized_output = Tokenizer::new();
            let content = fs::read_to_string(entry.path()).unwrap_or("".to_string());
            tokenized_output.tokenize(content.as_str());

            for token in tokenized_output.tokens {
                println!("the tokens is {}, the id of is {}", token.1, token.0);
            }
        }
    }
}

pub fn file_content_parsed<'a>(file_name: &str) {
    search_file(file_name);
}
