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

    delimiter_stack.iter().for_each(|el| println!("{:?}", el));
    resolve_emphasis(&mut parsed_inline_elements, &mut delimiter_stack);

    // Remove all placeholders

    parsed_inline_elements
}

fn resolve_emphasis(elements: &mut Vec<MdInlineElement>, delimiter_stack: &mut [Delimiter]) {
    println!("Starting Element Set: {:?}", elements);
    for i in 0..delimiter_stack.len() {
        if !delimiter_stack[i].active || !delimiter_stack[i].can_close {
            continue;
        }

        // At this point we have a valid closer
        let mut closer = delimiter_stack[i].clone();

        for j in (0..i).rev() {
            println!("J = {j}");
            if !delimiter_stack[j].active || !delimiter_stack[j].can_open {
                continue;
            }

            let opener = delimiter_stack[j].clone();

            // Check if the opener and closer have the same delimiter
            if !closer.ch.eq(&opener.ch) {
                continue;
            }

            // Rule of 3: If the total length of the run is a multiple of 3 and both run lengths
            // are not divisible by 3, they are not valid for emphasis
            let length_total = closer.run_length + opener.run_length;
            if ((closer.can_open && closer.can_close) || (opener.can_open && opener.can_close))
                && (length_total % 3 == 0
                    && closer.run_length % 3 != 0
                    && opener.run_length % 3 != 0)
            {
                continue;
            }

            // Prefer making bold connections first
            let delimiters_used = if closer.run_length >= 2 && opener.run_length >= 2 {
                2
            } else {
                1
            };

            // Replace the placeholders with the new element
            let range_start = if opener.run_length > delimiters_used {
                opener.parsed_position + 1
            } else {
                opener.parsed_position
            };

            let range_end = if closer.run_length >= delimiters_used {
                closer.parsed_position
            } else {
                closer.parsed_position + 1
            };
            //
            // Map the delimiters used to bold/italic respectively
            let element_to_insert = match delimiters_used {
                2 => MdInlineElement::Bold {
                    content: elements[range_start + 1..range_end].to_vec(),
                },
                1 => MdInlineElement::Italic {
                    content: elements[range_start + 1..range_end].to_vec(),
                },
                _ => unreachable!(),
            };

            elements.splice(range_start..=range_end, vec![element_to_insert]);
            let num_elements_removed = range_end - range_start;

            // closer.parsed_position -= num_elements_removed;

            // Update the parsed positions of the delimiters
            (0..delimiter_stack.len()).for_each(|k| {
                if delimiter_stack[k].parsed_position > closer.parsed_position {
                    delimiter_stack[k].parsed_position -= num_elements_removed;
                }
            });

            delimiter_stack[i].run_length = delimiter_stack[i]
                .run_length
                .saturating_sub(delimiters_used);
            delimiter_stack[j].run_length = delimiter_stack[j]
                .run_length
                .saturating_sub(delimiters_used);

            if delimiter_stack[i].run_length == 0 {
                delimiter_stack[i].active = false;
            }
            if delimiter_stack[j].run_length == 0 {
                delimiter_stack[j].active = false;
            }
        }
    }

    // For all delimiters that are still active, replace the placeholders with Text elements
    delimiter_stack.iter_mut().for_each(|el| {
        if el.active && el.parsed_position < elements.len() {
            elements[el.parsed_position] = MdInlineElement::Text {
                content: el.ch.to_string(),
            };
        }
    });
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
