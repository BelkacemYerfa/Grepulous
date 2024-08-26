use std::{collections::HashMap, env, ffi::OsStr, fs, path::{Path, PathBuf}, process};

use ast::Tokenizer;
use ignore::WalkBuilder;
use walkdir::{DirEntry};

pub mod action;
pub mod ast;
pub mod ui;


pub fn file_content_parsed<'a>(file_name: &str) {
    FilteringResults::new().update_self_based_search(file_name);

}

// * ignore files inside of the gitignore stuff
pub fn file_content_parse() {
    FilteringResults::new().filter_git_gitignore(
        &env::current_dir().unwrap()
    );
}

type IgnoreDirEntry = DirEntry;

/*
 * approach to take after getting the file of the gitignore
 * 1 - categories them over dirs and files
 * 2 - filter first by dirs
 * 3 - filter the result of the dirs filtering by filtering the files
 * 4 - get the files from them and tokenize them , and start the search
*/

#[derive(Debug,Clone,PartialEq)]
struct FilteringResults {
    files: Vec<String>, // ? files to search in for the provided pattern by the user
    dirs: Vec<String>, // ? to be used for some ui stuff
}

impl FilteringResults {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_files(&self) -> Vec<String> {
        self.files.clone()
    }

    fn get_dirs(&self) -> Vec<String> {
        self.dirs.clone()
    }

    fn reset_self(&mut self) {
        self.files = vec![];
        self.dirs = vec![];
    }

    fn filter_git_gitignore(&mut self, root: &Path) {
        // Create a WalkBuilder
        let walker = WalkBuilder::new(root)
            .git_ignore(true)    // Use .gitignore files
            .git_global(false)   // Don't use global gitignore (.ignore files)
            .git_exclude(false)  // Don't use .git/info/
            .build();

        // Iterate over the entries
        for result in walker {
            match result {
                Ok(entry) => {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if entry.path().is_file() {
                        self.files.push(file_name)
                    } else if entry.path().is_dir() {
                        self.dirs.push(file_name)
                    }
                },
                Err(err) => eprintln!("ERROR: {}", err),
            }
        }
    }


    fn get_filtered_files(&self , file_name: &str) -> Vec<String> {
        let mut files = vec![];

        let file_comps = file_name.split(".").collect::<Vec<_>>();
        let extension = file_comps.last();

        for file in self.files.clone() {
            if file_name.starts_with("*") {
                if file.split(".").last() == Some(&extension.unwrap()) {
                    files.push(file)
                }
            } else {
                if file == file_name {
                    files.push(file)
                }
            }
        }

        files
    }

    fn update_self_based_search(&mut self, pattern : &str) {
        // * this is the result of searching for the file given by the user
        let root = env::current_dir().unwrap();
        self.filter_git_gitignore(&root);
        // * fix it: main.rs (false)
        let dir = PathBuf::from(pattern);

        // ! change this later (this is only made for testing purposes)
        if !dir.is_file() {
            self.files = self.get_filtered_files(pattern)
        } else if dir.is_dir() {
            // * update the dirs
            self.reset_self();
            // * update the files on where to search
            let root = env::current_dir().unwrap().join(&dir);
            self.filter_git_gitignore(&root);
        }

        println!("the new results are {:#?}", self);
    }
}
