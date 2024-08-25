use std::{collections::HashMap, env, ffi::OsStr, fs};

use ast::Tokenizer;
use walkdir::{DirEntry, WalkDir};

pub mod action;
pub mod ast;
pub mod ui;

fn ignore_files(entry: &DirEntry, files_to_ignore: &Vec<String>) {}

fn search_file(file_name: &str) {
    let workflow = env::current_dir().unwrap();
    let arr = file_name.split(".").collect::<Vec<&str>>();
    let mut current_entries = WalkDir::new(&workflow).into_iter().filter_map(|e| e.ok());
    if arr[0].contains("*") {
        let paths = current_entries.filter(|entry| {
            entry.file_type().is_file()
                && entry.path().extension().and_then(OsStr::to_str) == Some(arr[1])
        });
        for entry in paths {
            // * instead of retuning we need to parse it and do the search on it
            let mut tokenized_output = Tokenizer::new();
            let content = fs::read_to_string(entry.path()).unwrap_or("".to_string());
            tokenized_output.tokenize(content.as_str());

            /*  for token in tokenized_output.tokens {
                println!("the tokens is {}, the id of is {}", token.1, token.0);
            } */
        }
    } else {
        let target_path = current_entries.find(|entry| {
            entry.file_type().is_file()
                && entry.path().extension().and_then(OsStr::to_str) == Some(arr[1])
        });
        if let Some(entry) = target_path {
            let mut tokenized_output = Tokenizer::new();
            let content = fs::read_to_string(entry.path()).unwrap_or("".to_string());
            tokenized_output.tokenize(content.as_str());

            /* for token in tokenized_output.tokens {
                println!("the tokens is {}, the id of is {}", token.1, token.0);
            } */
        }
    }
}

pub fn file_content_parsed<'a>(file_name: &str) {
    search_file(file_name);
}

// * ignore files inside of the gitignore stuff
pub fn file_content_parse() {
    let workflow = env::current_dir().unwrap();
    let files_to_ignore = get_files_to_ignore();

    let current_entries = WalkDir::new(&workflow).into_iter().filter_map(|e| e.ok());
    let paths = current_entries.filter(|entry| entry.file_type().is_file());
    for entry in paths {
        // * instead of retuning we need to parse it and do the search on it+
        let mut tokenized_output = Tokenizer::new();
        let content = fs::read_to_string(entry.path()).unwrap_or("".to_string());
        tokenized_output.tokenize(content.as_str());
        /* for token in tokenized_output.tokens {
            println!("the tokens is {}, the id of is {}", token.1, token.0);
        } */
    }
}

/// get all the files in the gitignore file
fn get_files_to_ignore() -> Vec<String> {
    let workflow = env::current_dir().unwrap();
    let git_ignore = WalkDir::new(&workflow)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|entry| {
            entry.file_type().is_file()
                && entry.path().file_name().and_then(OsStr::to_str) == Some(".gitignore")
        });


    let mut tokenized_output = Tokenizer::new();
    if let Some(git_ignore) = git_ignore {
        let content = fs::read_to_string(&git_ignore.path()).unwrap_or("".to_string());
        tokenized_output.tokenize(&content);
        return tokenized_output
            .tokens
            .iter()
            .map(|e| e.1.to_string())
            .collect();
    } else {
        return tokenized_output
            .tokens
            .iter()
            .map(|e| e.1.to_string())
            .collect();
    }
}

/*
 * approach to take after getting the file of the gitignore
 * 1 - categories them over dirs and files
 * 2 - filter first by dirs
 * 3 - filter the result of the dirs filtering by filtering the files
 * 4 - get the files from them and tokenize them , and start the search
*/

struct GitIgnore<'a> {
    files: Vec<&'a str>,
    dirs: Vec<&'a str>,
}

impl<'a> GitIgnore<'a> {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_files(&self) -> Vec<&'a str> {
        self.files.clone()
    }

    fn get_dirs(&self) -> Vec<&'a str> {
        self.dirs.clone()
    }

    fn get_gitignore(&self) -> Vec<String> {
        let workflow = env::current_dir().unwrap();
        let git_ignore = WalkDir::new(&workflow)
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|entry| {
                entry.file_type().is_file()
                    && entry.path().file_name().and_then(OsStr::to_str) == Some(".gitignore")
            });

        let mut tokenized_output = Tokenizer::new();
        if let Some(git_ignore) = git_ignore {
            let content = fs::read_to_string(&git_ignore.path()).unwrap_or("".to_string());
            tokenized_output.tokenize(&content);
            return tokenized_output
                .tokens
                .iter()
                .map(|e| e.1.to_string())
                .collect();
        } else {
            return tokenized_output
                .tokens
                .iter()
                .map(|e| e.1.to_string())
                .collect();
        }
    }

    fn clean_gitignore_content(&self) -> Vec<String> {
        let mut results = self.get_gitignore();

        if results.is_empty() {
            return results;
        }

        let cleaning = |e : &String| {
            e.starts_with("#") || e.trim().is_empty()
        };

        results.retain(|e| !cleaning(e));
        results.into_iter().map(|e| e.trim().to_string()).collect::<Vec<_>>()
    }


}
