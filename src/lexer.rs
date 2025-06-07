use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Asterisk,
    DoubleAsterisk,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
    Escape(char),
    Newline,
}
pub fn tokenize(markdown_line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let str_len = markdown_line.graphemes(true).count();
    let chars = Vec::from_iter(markdown_line.graphemes(true));
    // Loop through each character, and perform foward lookups for *
    let mut recent_emphasis: Token = Token::Whitespace;
    let mut i = 0;
    while i < str_len {
        match chars[i] {
            "*" => {
                // Perform forward lookup for another *
                if (i + 1 < str_len) && chars[i + 1] == "*" && recent_emphasis != Token::Asterisk {
                    tokens.push(Token::DoubleAsterisk);
                    i += 1;
                } else {
                    tokens.push(Token::Asterisk);
                    if recent_emphasis == Token::Asterisk {
                        recent_emphasis = Token::Whitespace;
                    } else {
                        recent_emphasis = Token::Asterisk;
                    }
                }
            }
            " " => tokens.push(Token::Whitespace),
            // Note that graphemes() returns strings because graphemes can consist of things like a
            // char + a modifier
            _ => tokens.push(Token::Text(String::from(chars[i]))),
        }

        i += 1;
    }

    tokens
}
