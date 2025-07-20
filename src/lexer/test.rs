use std::sync::Once;

use crate::config::init_config;
use crate::lexer::{Token::*, *};

static INIT: Once = Once::new();

fn init_test_config() {
    INIT.call_once(|| {
        init_config("config.toml").expect("Failed to initialize test config");
    });
}

#[test]
fn text() {
    init_test_config();
    assert_eq!(tokenize("Hello"), vec![Text(String::from("Hello"))]);
}

#[test]
fn punctuation() {
    init_test_config();
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
    init_test_config();
    assert_eq!(tokenize(" "), vec![Whitespace]);
}

#[test]
fn newline() {
    init_test_config();
    assert_eq!(tokenize("\n"), vec![Newline]);
}

#[test]
fn italic() {
    init_test_config();
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
    init_test_config();
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
    init_test_config();
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
    init_test_config();
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
    init_test_config();
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
    init_test_config();
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
    init_test_config();
    assert_eq!(tokenize("---"), vec![ThematicBreak]);
}

#[test]
fn code_tick() {
    init_test_config();
    assert_eq!(
        tokenize("`code`"),
        vec![CodeTick, Text(String::from("code")), CodeTick]
    );
}

#[test]
fn code_fence() {
    init_test_config();
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
    init_test_config();
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
    init_test_config();
    assert_eq!(tokenize("\t"), vec![Tab]);
}

#[test]
fn tab_via_spaces() {
    init_test_config();
    assert_eq!(tokenize("    "), vec![Tab]);
}

#[test]
fn blockquote() {
    init_test_config();
    assert_eq!(
        tokenize("> This is a blockquote."),
        vec![
            BlockQuoteMarker,
            Whitespace,
            Text(String::from("This")),
            Whitespace,
            Text(String::from("is")),
            Whitespace,
            Text(String::from("a")),
            Whitespace,
            Text(String::from("blockquote")),
            Punctuation(String::from("."))
        ]
    );
}

#[test]
fn raw_html_basic() {
    init_test_config();
    assert_eq!(tokenize("<br>"), vec![RawHtmlTag(String::from("<br>"))]);
}

#[test]
fn raw_html_with_attributes() {
    init_test_config();
    assert_eq!(
        tokenize("<img src=\"image.jpg\" alt=\"An image\">"),
        vec![RawHtmlTag(String::from(
            "<img src=\"image.jpg\" alt=\"An image\">"
        ))]
    );
}

#[test]
fn raw_inline_html() {
    init_test_config();
    assert_eq!(
        tokenize("This is <span>Inline HTML</span>"),
        vec![
            Text(String::from("This")),
            Whitespace,
            Text(String::from("is")),
            Whitespace,
            RawHtmlTag(String::from("<span>")),
            Text(String::from("Inline")),
            Whitespace,
            Text(String::from("HTML")),
            RawHtmlTag(String::from("</span>"))
        ]
    );
}

#[test]
fn unicode_mixed() {
    init_test_config();
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
