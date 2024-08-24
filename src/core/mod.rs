use std::{env, fs, path::{Path, PathBuf}};

use ast::Tokenizer;
use walkdir::WalkDir;

pub mod ast;
pub mod action;


/* fn search_file(file_name : &str) -> Option<PathBuf> {
  let workflow = env::current_dir().unwrap();
  let arr = file_name.split(".");



  let entry = WalkDir::new(&workflow)
              .into_iter()
              .find(|e| e.unwrap().file_name().to_str().unwrap() == file_name);

  if let Some(entry) = entry {
    return Some(entry.unwrap().path().to_path_buf())
  } else {
    None
  }


}

fn file_content_parsed<'a>(file_name:&str) -> Tokenizer<'a> {
  let path = search_file(file_name);
  if let Some(path) = path {
    let mut tokenized_output = Tokenizer::new();
    let content = fs::read_to_string(&path).unwrap_or("".to_string());
    if !content.is_empty() {
      tokenized_output.tokenize(content.as_str());
    }

    tokenized_output
  } else {
    Tokenizer::new()
  }
} */