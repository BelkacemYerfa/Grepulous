// * this code will contain a basic code for parsing our args through the command line
// * hint: use env::args() to build on top of

use regex::Regex;
use std::process;

use crate::core::{file_content_parse, file_content_parsed};

pub mod parser;


#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pattern: String,
    file: Option<String>,
}

impl Args {
    fn new() -> Self {
        Self {
            pattern: "".to_string(),
            file: None,
        }
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

    fn validate_pattern(&self) -> regex::Regex {
        // * validate if the regex pattern is valid or not
        let regex = Regex::new(&self.pattern);
        match regex {
            Err(err) => {
                eprintln!("This is a bad regex");
                eprintln!("{:#?}", err);
                process::exit(1)
            }
            Ok(regex) => {
                regex
            }
        }
    }

    fn executer(&self) {
        match &self.file {
            Some(file_name) => file_content_parsed(&file_name , self.validate_pattern()),
            None => file_content_parse(self.validate_pattern()),
        }
    }
}
