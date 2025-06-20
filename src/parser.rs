use crate::lexer::Token;
use crate::types::{Delimiter, MdBlockElement, MdInlineElement, TokenCursor};
use crate::utils::push_buffer_to_collection;

pub fn parse_blocks(markdown_lines: Vec<Vec<Token>>) -> Vec<MdBlockElement> {
    let mut block_elements: Vec<MdBlockElement> = Vec::new();

    for line in markdown_lines {
        let first_token = line.first();

        match first_token {
            Some(Token::Punctuation(string)) if string == "#" => {
                block_elements.push(parse_heading(line));
            }
            Some(Token::Text(_)) | Some(Token::Punctuation(_)) => {
                block_elements.push(MdBlockElement::Paragraph {
                    content: parse_inline(line),
                })
            }
            _ => {}
        }
    }

    block_elements
}

fn parse_codeblock(line: Vec<Token>) -> MdBlockElement {
    let mut code_content: Vec<String> = Vec::new();
    let mut language = None;
    let mut line_buffer: String = String::new();

    if let Some(Token::Text(string)) = line.get(1) {
        language = Some(string.clone());
    }

    for i in 2..line.len() {
        match line.get(i) {
            Some(Token::CodeFence) => {
                push_buffer_to_collection(&mut code_content, &mut line_buffer);

                break;
            }
            Some(Token::Text(string)) | Some(Token::Punctuation(string)) => {
                line_buffer.push_str(string);
            }
            Some(Token::Whitespace) => line_buffer.push(' '),
            Some(Token::Newline) => line_buffer.push('\n'),
            Some(Token::Escape(esc_char)) => {
                line_buffer.push_str(format!("\\{esc_char}").as_str());
            }
            Some(Token::CodeTick) => {
                // If we encounter a code tick, treat it as a text element
                line_buffer.push('`');
            }
            Some(Token::OpenParenthesis) => line_buffer.push('('),
            Some(Token::CloseParenthesis) => line_buffer.push(')'),
            Some(Token::OpenBracket) => line_buffer.push('['),
            Some(Token::CloseBracket) => line_buffer.push(']'),
            Some(Token::EmphasisRun { delimiter, length }) => {
                line_buffer.push_str(delimiter.to_string().repeat(*length).as_str())
            }
            _ => {}
        }
    }

    push_buffer_to_collection(&mut code_content, &mut line_buffer);


    MdBlockElement::CodeBlock {
        language,
        lines: code_content,
    }
}

fn parse_heading(line: Vec<Token>) -> MdBlockElement {
    let mut heading_level = 0;
    let mut i = 0;
    while let Some(token) = line.get(i) {
        match token {
            Token::Punctuation(string) => {
                if string == "#" {
                    heading_level += 1;
                } else {
                    break;
                }
            }
            _ => break,
        }
        i += 1;
    }

    if i >= line.len() || line.get(i) != Some(&Token::Whitespace) {
        return MdBlockElement::Paragraph {
            content: parse_inline(line),
        };
    }

    MdBlockElement::Header {
        level: heading_level,
        content: parse_inline(line[i + 1..].to_vec()),
    }
}

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
                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);

                delimiter_stack.push(Delimiter {
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
                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);

                // Search for the matching closing bracket
                // Recursively call parse_inline on the tokens between the brackets?
                let mut label: String = String::new();
                let mut uri: String = String::new();
                let mut inner_delimiter_stack: Vec<Delimiter> = Vec::new();
                let mut inner_parsed_elements: Vec<MdInlineElement> = Vec::new();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CloseBracket => {
                            push_buffer_to_collection(
                                &mut inner_parsed_elements,
                                &mut label.clone(),
                            );
                            break;
                        }
                        Token::EmphasisRun { delimiter, length } => {
                            push_buffer_to_collection(
                                &mut inner_parsed_elements,
                                &mut label.clone(),
                            );

                            inner_delimiter_stack.push(Delimiter {
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
                        Token::Text(string) | Token::Punctuation(string) => {
                            label.push_str(string.as_str())
                        }
                        Token::Escape(ch) => label.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => label.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                // If we didn't find a closing bracket, treat it as text
                if cursor.current() != Some(&Token::CloseBracket) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("[{label}"),
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
                        Token::Text(string) | Token::Punctuation(string) => {
                            uri.push_str(string.as_str())
                        }
                        Token::Escape(ch) => uri.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => uri.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                if cursor.current() != Some(&Token::CloseParenthesis) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("[{label}]({uri}"),
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
                        Token::CodeFence => {}
                    }

                    cursor.advance();
                }

                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);

                if cursor.current() != Some(&Token::CodeTick) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("`{code_content}`"),
                    });
                } else {
                    parsed_inline_elements.push(MdInlineElement::Code {
                        content: code_content,
                    });
                }
            }
            Token::Punctuation(string) if string == "!" => {
                if cursor.peek_ahead(1) != Some(&Token::OpenBracket) {
                    // If the next token is not an open bracket, treat it as text
                    buffer.push('!');
                    continue;
                }

                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);
                cursor.advance(); // Advance to the open bracket

                let mut alt_text: String = String::new();
                let mut uri: String = String::new();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CloseBracket => {
                            break;
                        }
                        Token::OpenParenthesis => alt_text.push('('),
                        Token::CloseParenthesis => alt_text.push(')'),
                        Token::Text(string) | Token::Punctuation(string) => {
                            alt_text.push_str(string.as_str())
                        }
                        Token::Escape(ch) => alt_text.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => alt_text.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                // If we didn't find a closing bracket, treat the whole inline as text
                if cursor.current() != Some(&Token::CloseBracket) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("![{alt_text}"),
                    });
                    continue;
                }

                // At this point we should have parentheses for the uri, otherwise treat it as a
                // text element
                if cursor.peek_ahead(1) != Some(&Token::OpenParenthesis) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("![{alt_text}]"),
                    });
                    continue;
                }

                cursor.advance();
                while let Some(next_token) = cursor.current() {
                    match next_token {
                        Token::CloseParenthesis => break,
                        Token::Text(string) | Token::Punctuation(string) => {
                            uri.push_str(string.as_str())
                        }
                        Token::Escape(ch) => uri.push_str(format!("\\{ch}").as_str()),
                        Token::Whitespace => uri.push(' '),
                        _ => {}
                    }

                    cursor.advance();
                }

                if cursor.current() != Some(&Token::CloseParenthesis) {
                    parsed_inline_elements.push(MdInlineElement::Text {
                        content: format!("![{alt_text}]({uri}"),
                    });
                } else {
                    parsed_inline_elements.push(MdInlineElement::Image { alt_text, url: uri });
                }
            }
            Token::Escape(esc_char) => buffer.push_str(format!("\\{esc_char}").as_str()),
            Token::Text(string) | Token::Punctuation(string) => buffer.push_str(string.as_str()),
            Token::Whitespace => buffer.push(' '),
            Token::CloseBracket => buffer.push(']'),
            Token::OpenParenthesis => buffer.push('('),
            Token::CloseParenthesis => buffer.push(')'),
            _ => push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer),
        }

        cursor.advance();
    }

    push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);

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

pub fn group_lines_to_blocks(mut tokenized_lines: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
    let mut blocks: Vec<Vec<Token>> = Vec::new();
    let mut current_block: Vec<Token> = Vec::new();
    let mut previous_block: Vec<Token>;
    let lines = tokenized_lines.iter_mut();
    let mut is_inside_code_block = false;
    for line in lines {
        previous_block = blocks.last().unwrap_or(&Vec::new()).to_vec();
        if is_inside_code_block && line.first() != Some(&Token::CodeFence) {
            // If we are inside a code block, then we just append the line to the current block
            previous_block.extend(line.to_owned());
            previous_block.push(Token::Newline);
            blocks.pop();
            blocks.push(previous_block.clone());
            continue;
        } else if is_inside_code_block && line.first() == Some(&Token::CodeFence) {
            // If we are inside a code block and the line starts with a code fence, then we end the
            // code block
            is_inside_code_block = false;
            previous_block.extend(line.to_owned());
            blocks.pop();
            blocks.push(previous_block.clone());
            continue;
        }

        match line.first() {
            Some(Token::Punctuation(string)) if string == "#" => {
                // For ATX headings, it must all be on one line
                blocks.push(line.to_owned());
            }
            Some(Token::Punctuation(string)) if string == "-" => {
                // Setext heading 2
                if let Some(previous_line_start) = previous_block.first() {
                    if matches!(previous_line_start, Token::Text(_)) {
                        previous_block.insert(0, Token::Punctuation(String::from("#")));
                        previous_block.insert(1, Token::Punctuation(String::from("#")));
                        previous_block.insert(2, Token::Whitespace);

                        // Swap previous block in
                        blocks.pop();
                        blocks.push(previous_block.clone());
                    }
                } else {
                    current_block.extend(line.to_owned());
                }
            }
            Some(Token::CodeTick) => {
                blocks.push(line.to_owned());
            Some(Token::CodeFence) => {
                if !is_inside_code_block {
                    is_inside_code_block = true;
                    current_block.extend(line.to_owned());
                } else {
                    is_inside_code_block = false;
                    current_block.extend(line.to_owned());
                    blocks.push(current_block.clone());
                    current_block.clear();
                }
            }
            Some(Token::Text(string)) if string == "=" => {
                // Setext heading 1
                if let Some(previous_line_start) = previous_block.first() {
                    // If it's text, then prepend the previous line with "# "
                    if matches!(previous_line_start, Token::Text(_)) {
                        previous_block.insert(0, Token::Punctuation(String::from("#")));
                        previous_block.insert(1, Token::Whitespace);

                        // Swap previous block in
                        blocks.pop();
                        blocks.push(previous_block.clone());
                    }
                } else {
                    current_block.extend(line.to_owned());
                }
            }
            Some(Token::Text(_)) => {
                if !previous_block.is_empty() {
                    if matches!(previous_block.first(), Some(Token::Text(_))) {
                        previous_block.push(Token::Whitespace);
                        previous_block.extend(line.to_owned());
                        blocks.pop();
                        blocks.push(previous_block.clone());
                    } else if matches!(previous_block.first(), Some(Token::Punctuation(_))) {
                        // If the previous block was a heading, then this is a new paragraph
                        current_block.extend(line.to_owned());
                    } else {
                        // If the previous block was empty, then this is a new paragraph
                        current_block.extend(line.to_owned());
                    }
                } else {
                    // If the previous block was empty, then this is a new paragraph
                    current_block.extend(line.to_owned());
                }
            }
            _ => {
                // Catch-all for everything else
                current_block.extend(line.to_owned());
            }
        }

        if !current_block.is_empty() {
            blocks.push(current_block.clone());
        }

        current_block.clear();
    }
    blocks
}

#[cfg(test)]
mod test;
