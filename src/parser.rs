use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum MdBlockElement {
    Header {
        level: u8,
        content: Vec<MdInlineElement>,
    },
    Paragraph {
        content: Vec<MdInlineElement>,
    },
    CodeBlock {
        lines: Vec<String>,
    },
    UnorderedList {
        items: Vec<MdListItem>,
    },
    HorizontalRule,
}

#[derive(Debug, PartialEq)]
pub struct MdListItem {
    content: Vec<MdBlockElement>,
}

#[derive(Debug, PartialEq)]
pub enum MdInlineElement {
    Text {
        content: String,
    },
    Bold {
        content: Vec<MdInlineElement>,
    },
    Italic {
        content: Vec<MdInlineElement>,
    },
    Link {
        text: Vec<MdInlineElement>,
        url: String,
    },
}

#[derive(Debug)]
pub struct TokenCursor {
    tokens: Vec<Token>,
    current_position: usize,
}

impl TokenCursor {
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_position)
    }

    fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_position + n)
    }

    fn advance(&mut self) {
        if self.current_position < self.tokens.len() {
            self.current_position += 1;
        }
    }

    fn position(&self) -> usize {
        self.current_position
    }

    fn is_at_eof(&self) -> bool {
        self.current_position > self.tokens.len()
    }
}

pub fn parse_inline(markdown_tokens: Vec<Token>) -> Vec<MdInlineElement> {
    let mut parsed_inline_elements: Vec<MdInlineElement> = Vec::new();


    parsed_inline_elements
}

#[cfg(test)]
mod test;
