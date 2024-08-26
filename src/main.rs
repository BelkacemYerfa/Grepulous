use std::env;

use cmd::parser::cmd_parser;
use ignore::WalkBuilder;

mod cmd;
mod core;

fn main() {
    cmd_parser();
}
