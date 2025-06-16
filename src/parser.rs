use crate::lexer::Token;
use crate::types::{Delimiter, MdInlineElement, TokenCursor};

pub fn parse_inline(markdown_tokens: Vec<Token>) -> Vec<MdInlineElement> {
    let mut parsed_inline_elements: Vec<MdInlineElement> = Vec::new();

    let mut cursor: TokenCursor = TokenCursor {
        tokens: markdown_tokens,
        current_position: 0,
    };

    let mut buffer: String = String::new();

    let mut current_token: &Token;
    while !cursor.is_at_eof() {
        current_token = cursor.current().expect("Token should be valid markdown");

        match current_token {
            Token::DoubleAsterisk => {
                parsed_inline_elements.push(MdInlineElement::Bold {
                    content: parse_bold(&mut cursor),
                });
            }
            Token::Escape(esc_char) => buffer.push_str(format!("\\{esc_char}").as_str()),
            Token::Text(string) => buffer.push_str(string),
            Token::Whitespace => buffer.push(' '),
            _ => push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer),
        }
        cursor.advance();
    }

    push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

    parsed_inline_elements
}

fn parse_bold(cursor: &mut TokenCursor) -> Vec<MdInlineElement> {
    cursor.advance();

    let mut inner_tokens: Vec<Token> = Vec::new();
    while let Some(token) = cursor.current() {
        if token == &Token::DoubleAsterisk {
            break;
        }

        inner_tokens.push(token.clone());
        cursor.advance();
    }

    parse_inline(inner_tokens)
}
fn push_buffer_to_elements(elements: &mut Vec<MdInlineElement>, buffer: &mut String) {
    if !&buffer.is_empty() {
        elements.push(MdInlineElement::Text {
            content: buffer.to_string(),
        });
        buffer.clear();
    }
}

#[cfg(test)]
mod test;
