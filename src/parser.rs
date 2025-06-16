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
            Token::OpenBracket => {
                push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

                // Search for the matching closing bracket
                // Recursively call parse_inline on the tokens between the brackets?
                let mut label: String = String::new();
                let mut uri: String = String::new();
                let mut inner_delimiter_stack: Vec<Delimiter> = Vec::new();
                let mut inner_parsed_elements: Vec<MdInlineElement> = Vec::new();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CloseBracket => {
                            push_buffer_to_elements(&mut inner_parsed_elements, &mut label);
                            break;
                        }
                        Token::EmphasisRun { delimiter, length } => {
                            push_buffer_to_elements(&mut inner_parsed_elements, &mut label);

                            inner_delimiter_stack.push(Delimiter {
                                token: Token::EmphasisRun {
                                    delimiter: *delimiter,
                                    length: *length,
                                },
                                run_length: *length,
                                ch: *delimiter,
                                token_position: cursor.position(),
                                parsed_position: inner_parsed_elements.len(),
                                active: true,
                                can_open: true,
                                can_close: true,
                            });

                            inner_parsed_elements.push(MdInlineElement::Placeholder);
                        }
                        Token::Text(string) => label.push_str(string.as_str()),
                        Token::Escape(ch) => label.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => label.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                // If we didn't find a closing bracket, treat it as text
                if cursor.current() != Some(&Token::CloseBracket) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("[{label}]"),
                    });
                    continue;
                }

                // At this point we should have parentheses for the uri, otherwise treat it as a
                // text element
                if cursor.peek_ahead(1) != Some(&Token::OpenParenthesis) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("[{label}]"),
                    });
                    continue;
                }

                cursor.advance();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CloseParenthesis => break,
                        Token::Text(string) => uri.push_str(string.as_str()),
                        Token::Escape(ch) => uri.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => uri.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                if cursor.current() != Some(&Token::CloseParenthesis) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("({uri})"),
                    });
                } else {
                    resolve_emphasis(&mut inner_parsed_elements, &mut inner_delimiter_stack);
                    parsed_inline_elements.push(MdInlineElement::Link {
                        text: inner_parsed_elements,
                        url: uri,
                    });
                }
            }
            Token::CodeTick => {
                // Search for a matching code tick, everything else is text
                cursor.advance();
                let mut code_content: String = String::new();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CodeTick => break,
                        Token::Text(string) | Token::Punctuation(string) => {
                            code_content.push_str(string)
                        }
                        Token::Escape(ch) => code_content.push_str(format!("\\{ch}").as_str()),
                        Token::OpenParenthesis => code_content.push('('),
                        Token::CloseParenthesis => code_content.push(')'),
                        Token::OpenBracket => code_content.push('['),
                        Token::CloseBracket => code_content.push(']'),
                        Token::EmphasisRun { delimiter, length } => {
                            code_content.push_str(delimiter.to_string().repeat(*length).as_str())
                        }
                        Token::Whitespace => code_content.push(' '),
                        Token::Newline => code_content.push('\n'),
                    }

                    cursor.advance();
                }

                if cursor.current() != Some(&Token::CodeTick) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("`{code_content}`"),
                    });
                } else {
                    parsed_inline_elements.push(MdInlineElement::CodeBlock {
                        content: code_content,
                    });
                }
            }
            Token::Escape(esc_char) => buffer.push_str(format!("\\{esc_char}").as_str()),
            Token::Text(string) | Token::Punctuation(string) => buffer.push_str(string.as_str()),
            Token::Whitespace => buffer.push(' '),
            Token::CloseBracket => buffer.push(']'),
            Token::OpenParenthesis => buffer.push('('),
            Token::CloseParenthesis => buffer.push(')'),
            _ => push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer),
        }

        cursor.advance();
    }

    push_buffer_to_elements(&mut parsed_inline_elements, &mut buffer);

    delimiter_stack
        .iter_mut()
        .for_each(|el| el.classify_flanking(&cursor.tokens));

    resolve_emphasis(&mut parsed_inline_elements, &mut delimiter_stack);

    // Remove all placeholders

    parsed_inline_elements
}

fn resolve_emphasis(elements: &mut Vec<MdInlineElement>, delimiter_stack: &mut [Delimiter]) {
    for i in 0..delimiter_stack.len() {
        if !delimiter_stack[i].active || !delimiter_stack[i].can_close {
            continue;
        }

        // At this point we have a valid closer
        let closer = delimiter_stack[i].clone();

        for j in (0..i).rev() {
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
