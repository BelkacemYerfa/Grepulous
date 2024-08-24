// * this code will contain a basic code for parsing our args through the command line
// * hint: use env::args() to build on top of

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
        self.clone()
    }

    fn set_file(&mut self, file: Option<String>) -> Self {
        self.file = file;
        self.clone()
    }
}
