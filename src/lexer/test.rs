use crate::lexer::{Token::*, *};

#[test]
fn test_lexer_text() {
    assert_eq!(tokenize("Hello"), vec![Text(String::from("Hello"))]);
}

#[test]
fn test_lexer_punctuation() {
    assert_eq!(
        tokenize(".-..-,"),
        vec![
            Punctuation(String::from(".")),
            Punctuation(String::from("-")),
            Punctuation(String::from(".")),
            Punctuation(String::from(".")),
            Punctuation(String::from("-")),
            Punctuation(String::from(","))
        ]
    )
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
        vec![
            EmphasisRun {
                delimiter: '*',
                length: 1
            },
            Text(String::from("italic")),
            EmphasisRun {
                delimiter: '*',
                length: 1
            }
        ]
    );
}

#[test]
fn test_lexer_bold() {
    assert_eq!(
        tokenize("**bold**"),
        vec![
            EmphasisRun {
                delimiter: '*',
                length: 2
            },
            Text(String::from("bold")),
            EmphasisRun {
                delimiter: '*',
                length: 2
            }
        ]
    );
}

#[test]
fn test_lexer_mixed_asterisks() {
    assert_eq!(
        tokenize("***bold + italic***"),
        vec![
            EmphasisRun {
                delimiter: '*',
                length: 3
            },
            Text(String::from("bold")),
            Whitespace,
            Text(String::from("+")),
            Whitespace,
            Text(String::from("italic")),
            EmphasisRun {
                delimiter: '*',
                length: 3
            },
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
            Text(String::from("https")),
            Punctuation(String::from(":")),
            Punctuation(String::from("/")),
            Punctuation(String::from("/")),
            Text(String::from("www")),
            Punctuation(String::from(".")),
            Text(String::from("example")),
            Punctuation(String::from(".")),
            Text(String::from("com")),
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
            EmphasisRun {
                delimiter: '*',
                length: 1
            },
            Text(String::from("italic")),
            Whitespace,
            EmphasisRun {
                delimiter: '*',
                length: 2
            },
            Text(String::from("bold+italic")),
            EmphasisRun {
                delimiter: '*',
                length: 3
            },
            CloseBracket,
            OpenParenthesis,
            Text(String::from("https")),
            Punctuation(String::from(":")),
            Punctuation(String::from("/")),
            Punctuation(String::from("/")),
            Text(String::from("www")),
            Punctuation(String::from(".")),
            Text(String::from("example")),
            Punctuation(String::from(".")),
            Text(String::from("com")),
            CloseParenthesis
        ]
    );
}

#[test]
fn test_lexer_unicode() {
    assert_eq!(
        tokenize("これは正解です。"),
        vec![
            Text(String::from("これは正解です")),
            Punctuation(String::from("。"))
        ]
    );
}

#[test]
fn test_lexer_unicode_mixed() {
    assert_eq!(
        tokenize("**これ** means \"This\"!"),
        vec![
            EmphasisRun {
                delimiter: '*',
                length: 2
            },
            Text(String::from("これ")),
            EmphasisRun {
                delimiter: '*',
                length: 2
            },
            Whitespace,
            Text(String::from("means")),
            Whitespace,
            Punctuation(String::from("\"")),
            Text(String::from("This")),
            Punctuation(String::from("\"")),
            Punctuation(String::from("!"))
        ]
    );
}
