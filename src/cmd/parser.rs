use std::{env, process};

use super::Args;

pub fn cmd_parser() -> Args {
    let mut args = env::args().skip(1);

    if args.len() > 3 {
        eprintln!("ERROR: max args is 3")
    }

    if args.len() < 1 {
        // * the needed arg is the pattern
        eprintln!("ERROR: we need at least 1 arg");
        process::exit(2)
    }

    let mut formatted_args = Args::new();

    formatted_args.set_pattern(args.next().unwrap_or_default());
    formatted_args.set_file(args.next());

    formatted_args.executer();
    formatted_args
}