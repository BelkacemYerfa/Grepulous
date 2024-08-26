use std::{
    env, ffi::OsStr, fs, path::{Path, PathBuf}
};

use lexer::Tokenizer;
use ignore::WalkBuilder;

pub mod action;
pub mod lexer;
pub mod ui;

/*
 * approach to take after getting the file of the gitignore
 * 1 - categories them over dirs and files
 * 2 - filter first by dirs
 * 3 - filter the result of the dirs filtering by filtering the files
 * 4 - get the files from them and tokenize them , and start the search
*/

#[derive(Debug, Clone)]
struct TargetFiles {
    file_name : String,
    file_path : PathBuf,
}

#[derive(Debug, Clone)]
struct FilteringResults(Vec<TargetFiles>);

impl FilteringResults {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn get_files(&self) -> Vec<TargetFiles> {
        self.0.clone()
    }

    fn reset_self(&mut self) {
        self.0 = vec![];
    }

    fn filter_git_gitignore(&mut self, root: &Path) {
        // Create a WalkBuilder
        let walker = WalkBuilder::new(root)
            .git_ignore(true) // Use .gitignore files
            .git_global(false) // Don't use global gitignore (.ignore files)
            .git_exclude(false) // Don't use .git/info/
            .build();

        // Iterate over the entries
        for result in walker {
            match result {
                Ok(entry) => {
                    if entry.path().is_file() {
                        let target_file = TargetFiles {
                            file_name : entry.file_name().to_string_lossy().to_string(),
                            file_path : entry.path().to_path_buf()
                        };
                        self.0.push(target_file)
                    }
                }
                Err(err) => eprintln!("ERROR: {}", err),
            }
        }
    }

    fn get_filtered_files(&self, file_name: &str) -> Vec<TargetFiles> {
        let mut files = vec![];

        let file_comps = file_name.split(".").collect::<Vec<_>>();
        let extension = file_comps.last();

        for file in self.0.clone() {
            if file_name.starts_with("*") {
                if file.file_path.extension().and_then(OsStr::to_str) == Some(&extension.unwrap()) {
                    files.push(file)
                }
            } else {
                if file.file_name == file_name.to_string() {
                    files.push(file)
                }
            }
        }

        files
    }

    fn update_self_based_search(&mut self, pattern: &str) {
        // * this is the result of searching for the file given by the user
        let root = env::current_dir().unwrap();
        self.filter_git_gitignore(&root);
        // * fix it: main.rs (false)
        let dir = PathBuf::from(pattern);

        // ! change this later (this is only made for testing purposes)
        if dir.is_file() {
            self.0 = self.get_filtered_files(pattern)
        } else if dir.is_dir() {
            // * update the dirs
            self.reset_self();
            // * update the files on where to search
            let root = env::current_dir().unwrap().join(&dir);
            self.filter_git_gitignore(&root);
        }

    }
}

pub fn file_content_parsed(file_name: &str, pattern : regex::Regex) {
    let mut target_files = FilteringResults::new();
    target_files.update_self_based_search(file_name);

    for file in target_files.0 {
        let mut tokenizer = Tokenizer::new();
        let content = fs::read_to_string(&file.file_path).unwrap();
        tokenizer.tokenize(&content);
        let mut once = false;
        for line in tokenizer.tokens {
            if pattern.is_match(line.1) {
                if !once {
                    println!("----");
                    println!("file: {:?}", file.file_path);
                }
                once = true;
                println!("{}: {}", line.0 , line.1);
            }
        }
    }
}

// * ignore files inside of the gitignore stuff
pub fn file_content_parse(pattern : regex::Regex) {
    let mut target_files = FilteringResults::new();
    let root = env::current_dir().unwrap();
    target_files.filter_git_gitignore(&root);

    for file in target_files.0 {
        let mut tokenizer = Tokenizer::new();
        let content = fs::read_to_string(&file.file_path).unwrap();
        tokenizer.tokenize(&content);
        let mut once = false;
        for line in tokenizer.tokens {
            if pattern.is_match(line.1) {
                if !once {
                    println!("----");
                    println!("file: {:?}", file.file_path);
                }
                once = true;
                println!("{}: {}", line.0 , line.1);
            }
        }
    }
}
