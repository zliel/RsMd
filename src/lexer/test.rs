use crate::lexer::{Token::*, *};

#[test]
fn text() {
    assert_eq!(tokenize("Hello"), vec![Text(String::from("Hello"))]);
}

#[test]
fn punctuation() {
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
fn whitespace() {
    assert_eq!(tokenize(" "), vec![Whitespace]);
}

#[test]
fn newline() {
    assert_eq!(tokenize("\n"), vec![Newline]);
}

#[test]
fn italic() {
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
fn bold() {
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
fn mixed_asterisks() {
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
fn link() {
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
fn emphasis_in_link() {
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
fn unicode() {
    assert_eq!(
        tokenize("これは正解です。"),
        vec![
            Text(String::from("これは正解です")),
            Punctuation(String::from("。"))
        ]
    );
}

#[test]
fn thematic_break() {
    assert_eq!(tokenize("---"), vec![ThematicBreak]);
}

#[test]
fn code_tick() {
    assert_eq!(
        tokenize("`code`"),
        vec![CodeTick, Text(String::from("code")), CodeTick]
    );
}

#[test]
fn code_fence() {
    assert_eq!(
        tokenize("```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```"),
        vec![
            CodeFence,
            Text(String::from("rust")),
            Newline,
            Text(String::from("fn")),
            Whitespace,
            Text(String::from("main")),
            OpenParenthesis,
            CloseParenthesis,
            Whitespace,
            Punctuation(String::from("{")),
            Newline,
            Tab,
            Text(String::from("println")),
            Punctuation(String::from("!")),
            OpenParenthesis,
            Punctuation(String::from("\"")),
            Text(String::from("Hello")),
            Punctuation(String::from(",")),
            Whitespace,
            Text(String::from("world")),
            Punctuation(String::from("!")),
            Punctuation(String::from("\"")),
            CloseParenthesis,
            Punctuation(String::from(";")),
            Newline,
            Punctuation(String::from("}")),
            Newline,
            CodeFence
        ]
    )
}

#[test]
fn escape_sequence() {
    assert_eq!(
        tokenize("\\*escaped asterisk\\*"),
        vec![
            Escape(String::from("*")),
            Text(String::from("escaped")),
            Whitespace,
            Text(String::from("asterisk")),
            Escape(String::from("*"))
        ]
    );
}

#[test]
fn tab() {
    assert_eq!(tokenize("\t"), vec![Tab]);
}

#[test]
fn tab_via_spaces() {
    assert_eq!(tokenize("    "), vec![Tab]);
}

#[test]
fn unicode_mixed() {
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
