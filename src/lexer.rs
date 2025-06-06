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
