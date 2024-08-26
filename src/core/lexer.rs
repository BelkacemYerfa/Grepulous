use indexmap::IndexMap;

/// This contains the matched results from the file content
struct Matched<'a>(Vec<&'a str>);

/// MetaData if the line that matched the regex pattern provided
struct MetaData<'a> {
    project: &'a str,
    file_name: &'a str,
    lang: &'a str,
    matched: Matched<'a>,
}

type Token<'a> = (String, &'a str);

#[derive(Debug, Clone, PartialEq)]
pub struct Tokenizer<'a> {
    pub tokens: IndexMap<usize, &'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Self {
        Self {
            tokens: IndexMap::new(),
        }
    }

    pub fn tokenize(&mut self, content: &'a str) -> Self {
        let content = content.split("\n").collect::<Vec<_>>().into_iter();

        for (line_id, line_content) in content.enumerate() {
            if !line_content.trim().is_empty() {
                self.tokens.insert(line_id, line_content.trim());
            }
        }

        self.clone()
    }

    pub fn next_token(&self) -> Option<Token<'a>> {
        todo!()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (String, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}