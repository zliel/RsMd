use crate::lexer::{Token::*, *};

#[test]
fn test_lexer_text() {
    assert_eq!(tokenize("Hello!"), vec![Text(String::from("Hello!"))]);
}

#[test]
fn test_lexer_whitespace() {
    assert_eq!(tokenize(" "), vec![Whitespace]);
}

#[test]
fn test_lexer_newline() {
    assert_eq!(tokenize("\n"), vec![Newline]);
}

#[test]
fn test_lexer_italic() {
    assert_eq!(
        tokenize("*italic*"),
        vec![Asterisk, Text(String::from("italic")), Asterisk]
    );
}

#[test]
fn test_lexer_bold() {
    assert_eq!(
        tokenize("**bold**"),
        vec![DoubleAsterisk, Text(String::from("bold")), DoubleAsterisk]
    );
}

#[test]
fn test_lexer_mixed_asterisks() {
    assert_eq!(
        tokenize("***bold + italic***"),
        vec![
            DoubleAsterisk,
            Asterisk,
            Text(String::from("bold")),
            Whitespace,
            Text(String::from("+")),
            Whitespace,
            Text(String::from("italic")),
            DoubleAsterisk,
            Asterisk
        ]
    );
}

#[test]
fn test_lexer_link() {
    assert_eq!(
        tokenize("More information available [here](https://www.example.com)"),
        vec![
            Text(String::from("More")),
            Whitespace,
            Text(String::from("information")),
            Whitespace,
            Text(String::from("available")),
            Whitespace,
            OpenBracket,
            Text(String::from("here")),
            CloseBracket,
            OpenParenthesis,
            Text(String::from("https://www.example.com")),
            CloseParenthesis
        ]
    );
}

#[test]
fn test_lexer_emphasis_in_link() {
    assert_eq!(
        tokenize("[*italic **bold+italic***](https://www.example.com)"),
        vec![
            OpenBracket,
            Asterisk,
            Text(String::from("italic")),
            Whitespace,
            DoubleAsterisk,
            Text(String::from("bold+italic")),
            DoubleAsterisk,
            Asterisk,
            CloseBracket,
            OpenParenthesis,
            Text(String::from("https://www.example.com")),
            CloseParenthesis
        ]
    );
}

#[test]
fn test_lexer_unicode() {
    assert_eq!(
        tokenize("これは正解です。"),
        vec![Text(String::from("これは正解です。"))]
    );
}

#[test]
fn test_lexer_unicode_mixed() {
    assert_eq!(
        tokenize("**これ** means \"This\"!"),
        vec![
            DoubleAsterisk,
            Text(String::from("これ")),
            DoubleAsterisk,
            Whitespace,
            Text(String::from("means")),
            Whitespace,
            Text(String::from("\"This\"!"))
        ]
    );
}
