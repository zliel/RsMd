use crate::utils::push_buffer_to_collection;
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
                push_buffer_to_collection(&mut tokens, &mut buffer);

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
            "`" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                if i + 2 < str_len && chars[i + 1] == "`" && chars[i + 2] == "`" {
                    tokens.push(Token::CodeFence);
                    i += 2;
                } else {
                    tokens.push(Token::CodeTick);
                }
            }
            "\\" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                if i + 1 < str_len {
                    tokens.push(Token::Escape(String::from(chars[i + 1])));
                    i += 1;
                } else {
                    buffer.push_str(chars[i]);
                }
            }
            "-" => {
                // Check for thematic break
                push_buffer_to_collection(&mut tokens, &mut buffer);

                if i + 2 < str_len && chars[i + 1] == "-" && chars[i + 2] == "-" {
                    tokens.push(Token::ThematicBreak);
                    i += 2;
                } else {
                    tokens.push(Token::Punctuation(String::from(chars[i])));
                }
            }
            "[" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::OpenBracket);
            }
            "]" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::CloseBracket);
            }
            "(" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::OpenParenthesis);
            }
            ")" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::CloseParenthesis);
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                // Check for valid ordered list marker
                if i + 2 < str_len && chars[i + 1] == "." && chars[i + 2] == " " {
                    // Check if the line STARTS with a number followed by a dot and space
                    if i == 0 || tokens.last() == Some(&Token::Tab) {
                        push_buffer_to_collection(&mut tokens, &mut buffer);
                        tokens.push(Token::OrderedListMarker(chars[i].to_owned() + chars[i + 1]));
                        // tokens.push(Token::Whitespace);
                        i += 2;
                        continue;
                    } else {
                        // If the line does not start with a number followed by a dot and space,
                        // treat it as a regular text token
                        buffer.push_str(chars[i]);
                    }
                }
            }
            "\t" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::Tab);
            }
            " " => {
                // Will be configurable later, but for now we'll stick to 4 spaces = 1 tab
                if i + 3 < str_len
                    && chars[i + 1] == " "
                    && chars[i + 2] == " "
                    && chars[i + 3] == " "
                {
                    push_buffer_to_collection(&mut tokens, &mut buffer);
                    tokens.push(Token::Tab);
                    i += 4;
                    continue;
                }

                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::Whitespace);
            }
            "" | "\n" => {
                push_buffer_to_collection(&mut tokens, &mut buffer);

                tokens.push(Token::Newline);
            }
            // Note that graphemes() returns strings because graphemes can consist of things like a
            // char + a modifier
            _ if is_punctuation(chars[i]) => {
                push_buffer_to_collection(&mut tokens, &mut buffer);
                tokens.push(Token::Punctuation(String::from(chars[i])));
            }
            _ => buffer.push_str(chars[i]),
        }

        i += 1;
    }

    // If the current buffer isn't empty when the loop is over, append it to the tokens vector
    push_buffer_to_collection(&mut tokens, &mut buffer);

    tokens
}

fn is_punctuation(input_str: &str) -> bool {
    let ch = input_str.chars().next().unwrap_or_default();
    input_str.chars().count() == 1 && (ch.is_punctuation() || ch.is_symbol_currency())
}

#[cfg(test)]
mod test;
