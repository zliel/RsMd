use crate::lexer::Token;
use crate::types::{Delimiter, MdInlineElement, TokenCursor};

pub fn parse_inline(markdown_tokens: Vec<Token>) -> Vec<MdInlineElement> {
    let mut parsed_inline_elements: Vec<MdInlineElement> = Vec::new();

    let mut cursor: TokenCursor = TokenCursor {
        tokens: markdown_tokens,
        current_position: 0,
    };

    let mut delimiter_stack: Vec<Delimiter> = Vec::new();

    let mut buffer: String = String::new();

    let mut current_token: Token;
    while !cursor.is_at_eof() {
        current_token = cursor
            .current()
            .expect("Token should be valid markdown")
            .clone();

        match current_token {
            Token::EmphasisRun { delimiter, length } => {
                push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

                delimiter_stack.push(Delimiter {
                    token: Token::EmphasisRun { delimiter, length },
                    run_length: length,
                    ch: delimiter,
                    token_position: cursor.position(),
                    parsed_position: parsed_inline_elements.len(),
                    active: true,
                    can_open: true,
                    can_close: true,
                });

                parsed_inline_elements.push(MdInlineElement::Placeholder);
            }
            Token::Escape(esc_char) => buffer.push_str(format!("\\{esc_char}").as_str()),
            Token::Text(string) | Token::Punctuation(string) => buffer.push_str(string.as_str()),
            Token::Whitespace => buffer.push(' '),
            _ => push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer),
        }

        cursor.advance();
    }

    push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

    delimiter_stack
        .iter_mut()
        .for_each(|el| el.classify_flanking(&cursor.tokens));

    parsed_inline_elements
}

        }

    }

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
