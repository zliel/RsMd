use unicode_categories::UnicodeCategories;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    EmphasisRun { delimiter: char, length: usize },
    Punctuation(String),
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
    Escape(String),
    Newline,
}

pub fn tokenize(markdown_line: &str) -> Vec<Token> {
    if markdown_line.is_empty() {
        return vec![Token::Newline];
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: String = String::new();

    let str_len = markdown_line.graphemes(true).count();
    let chars = Vec::from_iter(markdown_line.graphemes(true));

    // Loop through each character, and perform foward lookups for *
    let mut i = 0;
    while i < str_len {
        match chars[i] {
            "*" | "_" => {
                // if the current buffer isn't empty, append a Text token to the Vec<Token>
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                let delimiter = chars[i];
                let mut run_length = 1;
                while i + run_length < str_len && chars[i + run_length] == delimiter {
                    run_length += 1;
                }

                tokens.push(Token::EmphasisRun {
                    delimiter: delimiter.chars().next().unwrap(),
                    length: run_length,
                });

                i += run_length - 1;
            }
            "\\" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                if i + 1 < str_len {
                    tokens.push(Token::Escape(String::from(chars[i + 1])));
                    i += 1;
                } else {
                    buffer.push_str(chars[i]);
                }
            }
            "[" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::OpenBracket);
            }
            "]" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::CloseBracket);
            }
            "(" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::OpenParenthesis);
            }
            ")" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::CloseParenthesis);
            }
            " " => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::Whitespace);
            }
            "" | "\n" => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);

                tokens.push(Token::Newline);
            }
            // Note that graphemes() returns strings because graphemes can consist of things like a
            // char + a modifier
            _ if is_punctuation(chars[i]) => {
                push_buffer_to_tokens(&mut tokens, &mut buffer);
                tokens.push(Token::Punctuation(String::from(chars[i])));
            }
            _ => buffer.push_str(chars[i]),
        }

        i += 1;
    }

    // If the current buffer isn't empty when the loop is over, append it to the tokens vector
    push_buffer_to_tokens(&mut tokens, &mut buffer);

    tokens
}

fn is_punctuation(input_str: &str) -> bool {
    let ch = input_str.chars().next().unwrap_or_default();
    input_str.chars().count() == 1 && (ch.is_punctuation() || ch.is_symbol_currency())
}

fn push_buffer_to_tokens(tokens: &mut Vec<Token>, buffer: &mut String) {
    if !&buffer.is_empty() {
        tokens.push(Token::Text(buffer.to_string()));
        buffer.clear();
    }
}

#[cfg(test)]
mod test;
