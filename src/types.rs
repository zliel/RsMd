//! This module defines the types used in the markdown parser, including tokens, inline elements,
//! block elements, and a cursor for navigating through tokens.

/// Represents the different types of tokens that can be found in a markdown line.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    EmphasisRun { delimiter: char, length: usize },
    Punctuation(String),
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    OrderedListMarker(String),
    Whitespace,
    CodeTick,
    CodeFence,
    ThematicBreak,
    Escape(String),
    Tab,
    Newline,
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token::Text(s.to_string())
    }
}

/// Represents block-level markdown elements.
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
    OrderedList {
        items: Vec<MdListItem>,
    },
}

/// Represents a list item in markdown, which can contain block elements.
///
/// # Fields
///
/// * `content` - The content of the list item, which can be any block-level markdown element.
#[derive(Debug, PartialEq)]
pub struct MdListItem {
    pub content: MdBlockElement,
}

/// Represents inline markdown elements (text, bold/italic, link, etc.)
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

/// Cursor for navigating through a vector of tokens
///
/// This struct provides methods to access the current token, peek ahead or behind, and advance the
/// cursor position.
///
/// # Fields
///
/// * `tokens` - A vector of tokens to navigate through.
/// * `current_position` - The current position of the cursor within the token vector.
#[derive(Debug)]
pub struct TokenCursor {
    pub tokens: Vec<Token>,
    pub current_position: usize,
}

impl TokenCursor {
    /// Returns the current token, if any.
    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current_position)
    }

    /// Returns the nth next token, if any.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of tokens to look ahead.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the token if it exists, or `None` if it is out of
    /// bounds.
    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_position + n)
    }

    /// Returns the nth previous token, if any.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of tokens to look behind.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the token if it exists, or `None` if it is out of
    pub fn _peek_behind(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_position - n)
    }

    /// Moves the cursor forward one position.
    pub fn advance(&mut self) {
        if self.current_position < self.tokens.len() {
            self.current_position += 1;
        }
    }

    /// Sets the cursor's position to the specified position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position to set the cursor to.
    ///
    /// # Panics
    ///
    /// Panics if the position is out of bounds for the token list.
    pub fn _set_position(&mut self, pos: usize) {
        if pos < self.tokens.len() {
            self.current_position = pos;
        } else {
            panic!("Position {pos} is out of bounds for the TokenCursor");
        }
    }

    /// Returns the current position of the cursor.
    pub fn position(&self) -> usize {
        self.current_position
    }

    /// Returns whether the cursor is at the end of the token list.
    pub fn is_at_eof(&self) -> bool {
        self.current_position >= self.tokens.len()
    }
}

/// Manages Delimiter runs in a markdown document.
/// A delimiter run is a sequence of the same character (e.g., `*`, `_`, `~`) that can be used for
/// bold/italic writing.
///
/// # Fields
///
/// * `ch` - The character that represents the delimiter (e.g., `*`, `_`, `~`).
/// * `run_length` - The number of times the delimiter character appears in a row.
/// * `token_position` - The position of the first token in this delimiter run.
/// * `parsed_position` - The position in the Vec<MdInlineElement> where the content of this
///   delimiter run will be stored.
/// * `active` - Whether this delimiter run is currently active (i.e., it has not been closed).
/// * `can_open` - Whether this delimiter can open a new emphasis run (e.g., it is left-flanking).
/// * `can_close` - Whether this delimiter can close an existing emphasis run (e.g., it is
///   right-flanking).
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
    /// For exmample, it is left flanking if it's not followed by non-whitespace, and either:
    /// 1. Not followed by punctuation
    /// 2. Followed by punctuation and
    ///
    /// Modifies the `can_open` and `can_close` fields in-place based on the classification.
    ///
    /// See https://spec.commonmark.org/0.31.2/#left-flanking-delimiter-run for more information.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of tokens to classify the delimiter against.
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

        // Apply Rule of 3 and underscore restrictions
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

/// Helper function to determine if a token is whitespace or newline.
///
/// # Arguments
///
/// * `token` - The token to check.
fn is_whitespace(token: &Token) -> bool {
    matches!(token, Token::Newline | Token::Whitespace)
}

/// Helper function to determine if a token is punctuation.
///
/// # Arguments
///
/// * `token` - The token to check.
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
