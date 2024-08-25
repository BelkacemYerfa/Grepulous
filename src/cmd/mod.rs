// * this code will contain a basic code for parsing our args through the command line
// * hint: use env::args() to build on top of

use regex::Regex;
use std::process;

use crate::core::{file_content_parse, file_content_parsed};

pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flags {
    Insensitive, // ? -i flag
    Count,       // ? -c flag
    Inverting,   // ? -v flag
    LineNum,     // ? -n flag
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    options: Option<Flags>,
    pattern: String,
    file: Option<String>,
}

impl Args {
    fn new() -> Self {
        Self {
            options: None,
            pattern: "".to_string(),
            file: None,
        }
    }

    fn set_options(&mut self, options: Option<Flags>) -> Self {
        self.options = options;
        self.clone()
    }

    fn set_pattern(&mut self, pattern: String) -> Self {
        self.pattern = pattern;
        self.validate_pattern();
        self.clone()
    }

    fn set_file(&mut self, file: Option<String>) -> Self {
        self.file = file;
        self.clone()
    }

    fn validate_pattern(&self) {
        // * validate if the regex pattern is valid or not
        let regex = Regex::new(&self.pattern);
        match regex {
            Err(err) => {
                println!("This is a bad regex to begin with {:#?}", err);
                process::exit(1)
            }
            _ => {}
        }
    }

    fn executer(&self) {
        match &self.file {
            Some(file_name) => file_content_parsed(&file_name),
            None => file_content_parse(),
        }
    }
}
