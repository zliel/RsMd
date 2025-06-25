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
        language: Option<String>,
        lines: Vec<String>,
    },
    ThematicBreak,
    UnorderedList {
        items: Vec<MdListItem>,
    },
}

#[derive(Debug, PartialEq)]
pub struct MdListItem {
    pub content: MdBlockElement,
}

#[derive(Debug, PartialEq, Clone)]
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
        title: Option<String>,
        url: String,
    },
    Image {
        alt_text: String,
        title: Option<String>,
        url: String,
    },
    Code {
        content: String,
    },
    Placeholder,
}

impl From<String> for MdInlineElement {
    fn from(s: String) -> Self {
        MdInlineElement::Text {
            content: s.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TokenCursor {
    pub tokens: Vec<Token>,
    pub current_position: usize,
}

impl TokenCursor {
    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_position)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_position + n)
    }

    pub fn peek_behind(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_position - n)
    }

    pub fn advance(&mut self) {
        if self.current_position < self.tokens.len() {
            self.current_position += 1;
        }
    }

    pub fn set_position(&mut self, pos: usize) {
        if pos < self.tokens.len() {
            self.current_position = pos;
        } else {
            panic!("Position {pos} is out of bounds for the TokenCursor");
        }
    }

    pub fn position(&self) -> usize {
        self.current_position
    }

    pub fn is_at_eof(&self) -> bool {
        self.current_position >= self.tokens.len()
    }
}

/// Manages Delimiter positions
/// - Token is the token type
/// - usize is the current position
#[derive(Debug, Clone)]
pub struct Delimiter {
    pub ch: char,
    pub run_length: usize,
    pub token_position: usize,
    pub parsed_position: usize,
    pub active: bool,
    pub can_open: bool,  //Must be left-flanking
    pub can_close: bool, //Must be right-flanking
}

impl Delimiter {
    /// Determines whether a delimiter is "Left", "Right", or "Both" flanking
    /// It is left flanking if it's not followed by non-whitespace, and either:
    /// 1. Not followed by punctuation
    /// 2. Followed by punctuation and
    pub fn classify_flanking(&mut self, tokens: &[Token]) {
        let before = if self.token_position > 0 {
            Some(&tokens[self.token_position - 1])
        } else {
            None
        };
        // println!("Before token: {:?}", before);

        let after = tokens.get(self.token_position + 1);
        // println!("After token: {:?}", after);

        let followed_by_whitespace = after.is_none_or(is_whitespace);
        let followed_by_punctuation = after.is_some_and(is_punctuation);

        let preceded_by_whitespace = before.is_none_or(is_whitespace);
        let preceded_by_punctuation = before.is_some_and(is_punctuation);

        let is_left_flanking = if followed_by_whitespace {
            false
        } else if !followed_by_punctuation {
            true
        } else {
            preceded_by_whitespace || preceded_by_punctuation
        };

        let is_right_flanking = if preceded_by_whitespace {
            false
        } else if !preceded_by_punctuation {
            true
        } else {
            followed_by_whitespace || followed_by_punctuation
        };

        let delimiter_char = self.ch;

        // Apply Rule of 3 (underscore restrictions)
        let is_underscore = delimiter_char == '_';

        if is_underscore {
            self.can_open = is_left_flanking && (!is_right_flanking || followed_by_punctuation);

            self.can_close = is_right_flanking && (!is_left_flanking || followed_by_punctuation);
        } else {
            self.can_open = is_left_flanking;
            self.can_close = is_right_flanking;
        }
    }
}

fn is_whitespace(token: &Token) -> bool {
    matches!(token, Token::Newline | Token::Whitespace)
}

fn is_punctuation(token: &Token) -> bool {
    matches!(
        token,
        Token::Punctuation(_)
            | Token::EmphasisRun {
                delimiter: _,
                length: _
            }
            | Token::OpenBracket
            | Token::CloseBracket
            | Token::OpenParenthesis
            | Token::CloseParenthesis
    )
}
