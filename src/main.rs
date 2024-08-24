use std::env;
use std::io;
use std::process;

use cmd::parser::cmd_parser;

mod cmd;
mod core;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == "\\d" {
        return input_line.contains(|c: char| c.is_digit(10));
    } else if pattern == "\\w" {
        return input_line.contains(|c: char| c.is_alphanumeric());
    } else if pattern.starts_with("[^") && pattern.ends_with("]") {
        let chars = pattern[2..pattern.len() - 1].chars().collect::<Vec<char>>();
        return !input_line.chars().any(|c| chars.contains(&c));
    } else if pattern.starts_with("[") && pattern.ends_with("]") {
        let chars = pattern[1..pattern.len() - 1].chars().collect::<Vec<char>>();
        return input_line.chars().any(|c| chars.contains(&c));
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    /* let typ = Options::Ignore;
    println!("{}", typ.converter()); */
    cmd_parser();
}

fn testing() {

    /* if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    } */

    /*  let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new(); */

    /* io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    } */
}
