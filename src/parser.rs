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
        self.current_position >= self.tokens.len()
    }
}

pub fn parse_inline(markdown_tokens: Vec<Token>) -> Vec<MdInlineElement> {
    let mut parsed_inline_elements: Vec<MdInlineElement> = Vec::new();

    let mut cursor: TokenCursor = TokenCursor {
        tokens: markdown_tokens,
        current_position: 0,
    };

    let mut buffer: String = String::new();

    let mut current_token: &Token;
    while !cursor.is_at_eof() {
        current_token = cursor.current().expect("Token should be valid markdown");

        match current_token {
            Token::Text(string) => buffer.push_str(string),
            Token::Whitespace => buffer.push(' '),
            _ => push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer),
        }
        cursor.advance();
    }

    push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

    parsed_inline_elements
}

fn push_buffer_to_elements(elements: &mut Vec<MdInlineElement>, buffer: &mut String) {
    if !&buffer.is_empty() {
        elements.push(MdInlineElement::Text {
            content: buffer.to_string(),
        });
        buffer.clear();
    }
}

#[cfg(test)]
mod test;
