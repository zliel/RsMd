//! This module contains the parser for converting tokenized Markdown lines into structured
//! Markdown elements.
//!
//! It provides functions to parse block-level elements like headings, lists, and code blocks,
//! as well as inline elements like links, images, and emphasis.

use crate::types::{Delimiter, MdBlockElement, MdInlineElement, MdListItem, Token, TokenCursor};
use crate::utils::push_buffer_to_collection;

/// Parses a vector of tokenized markdown lines into a vector of block-level Markdown elements.
///
/// # Arguments
///
/// * `markdown_lines` - A vector of vectors, where each inner vector contains tokens representing a line of markdown.
///
/// # Returns
///
/// A vector of parsed block-level Markdown elements.
pub fn parse_blocks(markdown_lines: Vec<Vec<Token>>) -> Vec<MdBlockElement> {
    let mut block_elements: Vec<MdBlockElement> = Vec::new();

    for line in markdown_lines {
        if let Some(element) = parse_block(line) {
            block_elements.push(element)
        }
    }

    block_elements
}

/// Parses a single line of tokens into a block-level Markdown element.
///
/// # Arguments
///
/// * `line` - A vector of tokens representing a single line of markdown.
///
/// # Returns
///
/// An Option<MdBlockElement>, returning `None` for empty lines
fn parse_block(line: Vec<Token>) -> Option<MdBlockElement> {
    let first_token = line.first();

    match first_token {
        Some(Token::Punctuation(string)) if string == "#" => Some(parse_heading(line)),
        Some(Token::Punctuation(string)) if string == "-" => {
            // Note that setext headings have already been handled in the group_lines_to_blocks
            // function by this point
            if line.len() == 1 {
                // If the line only contains a dash, then it is a thematic break
                Some(MdBlockElement::ThematicBreak)
            } else {
                Some(parse_unordered_list(line))
            }
        }
        Some(Token::OrderedListMarker(_)) => Some(parse_ordered_list(line)),
        Some(Token::CodeFence) => Some(parse_codeblock(line)),
        Some(Token::ThematicBreak) => Some(MdBlockElement::ThematicBreak),
        Some(Token::Newline) => None,
        _ => Some(MdBlockElement::Paragraph {
            content: parse_inline(line),
        }),
    }
}

/// Parses a vector of tokens representing an ordered list into an `MdBlockElement::OrderedList`.
///
/// Calls the more generic `parse_list` function, which parses nested list items
///
/// # Arguments
///
/// * `list` - A vector of tokens representing an ordered list.
///
/// # Returns
///
/// An `MdBlockElement` representing the ordered list.
fn parse_ordered_list(list: Vec<Token>) -> MdBlockElement {
    parse_list(
        list,
        |tokens| {
            matches!(
                tokens.first(),
                Some(Token::OrderedListMarker(_)) if tokens.get(1) == Some(&Token::Whitespace)
            )
        },
        |items| MdBlockElement::OrderedList { items },
    )
}

/// Parses a vector of tokens representing an unordered list into an `MdBlockElement::UnorderedList`.
///
/// Calls the more generic `parse_list` function, which parses nested list items
///
/// # Arguments
///
/// * `list` - A vector of tokens representing an unordered list.
///
/// # Returns
///
/// An `MdBlockElement` representing the unordered list.
fn parse_unordered_list(list: Vec<Token>) -> MdBlockElement {
    parse_list(
        list,
        |tokens| {
            matches!(tokens.first(), Some(Token::Punctuation(string)) if string == "-" && tokens.get(1) == Some(&Token::Whitespace)
            )
        },
        |items| MdBlockElement::UnorderedList { items },
    )
}

/// Generic list parser used to reduce code duplication between ordered and unordered lists.
///
/// Handles splitting lines, identifying list items, and parsing nested lists. The behavior is
/// determined by a predicate for identifying list items and a constructor for the resulting block.
///
/// # Arguments
///
/// * `list` - The tokens to parse.
/// * `is_list_item` - Predicate to identify a top-level list item.
/// * `make_block` - Constructor for the resulting `MdBlockElement`.
///
/// # Returns
///
/// An `MdBlockElement` representing either an ordered or unordered list, depending on the passed in constructor.
fn parse_list<F, G>(list: Vec<Token>, is_list_item: F, make_block: G) -> MdBlockElement
where
    F: Fn(&[Token]) -> bool,
    G: Fn(Vec<MdListItem>) -> MdBlockElement,
{
    let lists_split_by_newline = list
        .split(|token| *token == Token::Newline)
        .collect::<Vec<_>>();
    let mut list_items: Vec<MdListItem> = Vec::new();

    let mut i = 0;
    while i < lists_split_by_newline.len() {
        let line = lists_split_by_newline[i];
        if is_list_item(line) {
            let content_tokens = line[2..].to_vec();
            if let Some(content) = parse_block(content_tokens) {
                list_items.push(MdListItem { content })
            }

            // Check for consecutive tab-indented lines (nested list)
            let mut nested_lines: Vec<Vec<Token>> = Vec::new();
            let mut j = i + 1;
            while j < lists_split_by_newline.len() {
                let nested_line = lists_split_by_newline[j];
                if nested_line.first() == Some(&Token::Tab) {
                    let mut nested = nested_line.to_vec();
                    while !nested.is_empty() && nested[0] == Token::Tab {
                        nested.remove(0);
                    }
                    nested_lines.push(nested);
                    j += 1;
                } else {
                    break;
                }
            }

            if !nested_lines.is_empty() {
                // Flatten nested lines into a single Vec<Token> separated by Newline
                let mut nested_tokens: Vec<Token> = Vec::new();
                for (k, l) in nested_lines.into_iter().enumerate() {
                    if k > 0 {
                        nested_tokens.push(Token::Newline);
                    }
                    nested_tokens.extend(l);
                }

                // Recursively parse nested list, try ordered first, fallback to unordered
                let nested_block = if let Some(Token::OrderedListMarker(_)) = nested_tokens.first()
                {
                    parse_ordered_list(nested_tokens)
                } else {
                    parse_unordered_list(nested_tokens)
                };

                list_items.push(MdListItem {
                    content: nested_block,
                });

                i = j - 1; // Skip processed nested lines
            }
        }
        i += 1;
    }

    // Use the passed in constructor to create the List element
    make_block(list_items)
}

/// Parses a vector of tokens representing a code block into an `MdBlockElement::CodeBlock`.
///
/// Extracts the language (if specified) and the code content.
///
/// # Arguments
///
/// * `line` - A vector of tokens representing a code block.
///
/// # Returns
///
/// An `MdBlockElement` representing the code block.
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
            Some(Token::OrderedListMarker(string)) => line_buffer.push_str(string),
            Some(Token::Whitespace) => line_buffer.push(' '),
            Some(Token::Newline) => line_buffer.push('\n'),
            Some(Token::ThematicBreak) => line_buffer.push_str("---"),
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

/// Parses a vector of tokens representing a heading into an `MdBlockElement::Header`.
///
/// Determines the heading level and parses the heading content.
///
/// # Arguments
///
/// * `line` - A vector of tokens representing a heading line.
///
/// # Returns
///
/// An `MdBlockElement` representing the heading, or a paragraph if the heading is invalid.
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

    // At this point, we should be at a non-# token or the end of the line
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

/// Parses a vector of tokens into a vector of inline Markdown elements.
///
/// Handles emphasis, links, images, and code spans
///
/// # Arguments
///
/// * `markdown_tokens` - A vector of tokens representing inline markdown content.
///
/// # Returns
///
/// A vector of parsed inline Markdown elements.
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

                let link_element =
                    parse_link_type(&mut cursor, |label, title, url| MdInlineElement::Link {
                        text: label,
                        title,
                        url,
                    });
                parsed_inline_elements.push(link_element);
            }
            Token::CodeTick => {
                // Search for a matching code tick, everything else is text
                cursor.advance();
                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);

                let code_content = parse_code_span(&mut cursor);

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
                    cursor.advance();
                    continue;
                }

                push_buffer_to_collection(&mut parsed_inline_elements, &mut buffer);
                cursor.advance(); // Advance to the open bracket

                let image =
                    parse_link_type(&mut cursor, |label, title, url| MdInlineElement::Image {
                        alt_text: flatten_inline(label),
                        title,
                        url,
                    });

                parsed_inline_elements.push(image);
            }
            Token::Escape(esc_char) => buffer.push_str(format!("\\{esc_char}").as_str()),
            Token::Text(string) | Token::Punctuation(string) => buffer.push_str(string.as_str()),
            Token::OrderedListMarker(string) => buffer.push_str(string.as_str()),
            Token::Whitespace => buffer.push(' '),
            Token::CloseBracket => buffer.push(']'),
            Token::OpenParenthesis => buffer.push('('),
            Token::CloseParenthesis => buffer.push(')'),
            Token::ThematicBreak => buffer.push_str("---"),
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

/// Parses a code span starting from the current position of the cursor.
///
/// # Arguments
///
/// * `cursor` - A mutable reference to a `TokenCursor` that tracks the current position in the
///
/// # Returns
///
/// A string containing the content of the code span, excluding the opening and closing code ticks.
fn parse_code_span(cursor: &mut TokenCursor) -> String {
    let mut code_content: String = String::new();
    while let Some(next_token) = cursor.current() {
        match next_token {
            Token::CodeTick => break,
            Token::Text(string) | Token::Punctuation(string) => code_content.push_str(string),
            Token::OrderedListMarker(string) => code_content.push_str(string),
            Token::Escape(ch) => code_content.push_str(format!("\\{ch}").as_str()),
            Token::OpenParenthesis => code_content.push('('),
            Token::CloseParenthesis => code_content.push(')'),
            Token::OpenBracket => code_content.push('['),
            Token::CloseBracket => code_content.push(']'),
            Token::EmphasisRun { delimiter, length } => {
                code_content.push_str(delimiter.to_string().repeat(*length).as_str())
            }
            Token::Whitespace => code_content.push(' '),
            Token::Tab => code_content.push_str("    "), // 4 spaces for a tab,
            // will be changed via configuration later
            Token::Newline => code_content.push('\n'),
            Token::ThematicBreak => code_content.push_str("---"),
            Token::CodeFence => {}
        }

        cursor.advance();
    }

    code_content
}

/// Parses a link type (either a link or an image) from the current position of the cursor.
///
/// This function handles the parsing of the link label, URI, and optional title.
///
/// # Arguments
///
/// * `cursor` - A mutable reference to a `TokenCursor` that tracks the current position in the
///   token stream.
/// * `make_element` - A closure that takes the parsed label elements, optional title, and URI,
///   and returns an `MdInlineElement` representing the link or image.
///
/// # Returns
///
/// An `MdInlineElement` representing the parsed link or image.
fn parse_link_type<F>(cursor: &mut TokenCursor, make_element: F) -> MdInlineElement
where
    F: Fn(Vec<MdInlineElement>, Option<String>, String) -> MdInlineElement,
{
    let mut label_elements: Vec<MdInlineElement> = Vec::new();
    let mut label_buffer = String::new();
    let mut delimiter_stack: Vec<Delimiter> = Vec::new();
    while let Some(token) = cursor.current() {
        match token {
            Token::CloseBracket => {
                push_buffer_to_collection(&mut label_elements, &mut label_buffer);
                break;
            }
            Token::EmphasisRun { delimiter, length } => {
                push_buffer_to_collection(&mut label_elements, &mut label_buffer);
                delimiter_stack.push(Delimiter {
                    run_length: *length,
                    ch: *delimiter,
                    token_position: cursor.position(),
                    parsed_position: label_elements.len(),
                    active: true,
                    can_open: true,
                    can_close: true,
                });
                label_elements.push(MdInlineElement::Placeholder);
            }
            Token::Text(s) | Token::Punctuation(s) => label_buffer.push_str(s),
            Token::OrderedListMarker(s) => label_buffer.push_str(s),
            Token::Escape(ch) => label_buffer.push_str(format!("\\{ch}").as_str()),
            Token::Whitespace => label_buffer.push(' '),
            Token::ThematicBreak => label_buffer.push_str("---"),
            Token::OpenParenthesis => label_buffer.push('('),
            Token::CloseParenthesis => label_buffer.push(')'),
            _ => {}
        }
        cursor.advance();
    }

    resolve_emphasis(&mut label_elements, &mut delimiter_stack);

    // If we didn't find a closing bracket, treat it as text
    if cursor.current() != Some(&Token::CloseBracket) {
        return MdInlineElement::Text {
            content: format!("[{}", flatten_inline(label_elements)),
        };
    }

    // At this point we should have parentheses for the uri, otherwise treat it as a
    // text element
    if cursor.peek_ahead(1) != Some(&Token::OpenParenthesis) {
        cursor.advance();
        return MdInlineElement::Text {
            content: format!("[{}]", flatten_inline(label_elements)),
        };
    }

    cursor.advance(); // Move to '('

    let mut uri = String::new();
    let mut title = String::new();
    let mut is_building_title = false;
    let mut is_valid_title = true;
    let mut has_opening_quote = false;

    while let Some(token) = cursor.current() {
        if !is_building_title {
            match token {
                Token::CloseParenthesis => break,
                Token::Text(s) | Token::Punctuation(s) => uri.push_str(s),
                Token::OrderedListMarker(s) => uri.push_str(s),
                Token::Escape(ch) => uri.push_str(format!("\\{ch}").as_str()),
                Token::Whitespace => is_building_title = true,
                Token::ThematicBreak => uri.push_str("---"),
                _ => {}
            }
        } else {
            match token {
                Token::CloseParenthesis => break,
                Token::Punctuation(s) if s == "\"" => {
                    if has_opening_quote {
                        is_valid_title = true;
                        is_building_title = false;
                    } else {
                        has_opening_quote = true;
                        is_valid_title = false;
                    }
                }
                Token::Text(s) | Token::Punctuation(s) => title.push_str(s),
                Token::OrderedListMarker(s) => title.push_str(s),
                Token::Escape(ch) => title.push_str(format!("\\{ch}").as_str()),
                Token::EmphasisRun { delimiter, length } => {
                    title.push_str(delimiter.to_string().repeat(*length).as_str())
                }
                Token::OpenBracket => title.push('['),
                Token::CloseBracket => title.push(']'),
                Token::OpenParenthesis => title.push('('),
                Token::Tab => title.push('\t'),
                Token::Newline => title.push_str("\\n"),
                Token::Whitespace => title.push(' '),
                Token::CodeTick => title.push('`'),
                Token::CodeFence => title.push_str("```"),
                Token::ThematicBreak => title.push_str("---"),
            }
        }
        cursor.advance();
    }

    // If we didn't find a closing parenthesis or if the title is invalid, treat it as text
    if cursor.current() != Some(&Token::CloseParenthesis) {
        return MdInlineElement::Text {
            content: format!("[{}]({} ", flatten_inline(label_elements), uri),
        };
    } else if !title.is_empty() && !is_valid_title {
        return MdInlineElement::Text {
            content: format!("[{}]({} {})", flatten_inline(label_elements), uri, title),
        };
    }

    make_element(label_elements, Some(title).filter(|t| !t.is_empty()), uri)
}

/// Flattens a vector of inline Markdown elements into a single string.
///
/// # Arguments
///
/// * `elements` - A vector of inline Markdown elements to flatten.
///
/// # Returns
///
/// A string containing the concatenated content of all inline elements
fn flatten_inline(elements: Vec<MdInlineElement>) -> String {
    let mut result = String::new();
    for element in elements {
        match element {
            MdInlineElement::Text { content } => result.push_str(&content),
            MdInlineElement::Bold { content } => result.push_str(&flatten_inline(content)),
            MdInlineElement::Italic { content } => result.push_str(&flatten_inline(content)),
            MdInlineElement::Code { content } => result.push_str(&content),
            MdInlineElement::Link { text, .. } => result.push_str(&flatten_inline(text)),
            MdInlineElement::Image { alt_text, .. } => result.push_str(&alt_text),
            _ => {}
        }
    }
    result
}
/// Parses (resolves) emphasis in a vector of inline Markdown elements.
///
/// Modifies the elements in place to convert delimiter runs into bold or italic elements as appropriate.
///
/// # Arguments
///
/// * `elements` - A mutable reference to a vector of inline Markdown elements.
/// * `delimiter_stack` - A mutable reference to a slice of delimiters.
fn resolve_emphasis(elements: &mut Vec<MdInlineElement>, delimiter_stack: &mut [Delimiter]) {
    if delimiter_stack.len() == 1 {
        // If there is only one delimiter, it cannot be resolved to emphasis
        if delimiter_stack[0].active {
            elements[delimiter_stack[0].parsed_position] = MdInlineElement::Text {
                content: delimiter_stack[0].ch.to_string(),
            };
        }
        return;
    }

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

/// Groups adjacent tokenized lines into groups (blocks) for further parsing.
///
/// # Arguments
///
/// * `tokenized_lines` - A vector of vectors, where each inner vector contains tokens representing a line of markdown.
///
/// # Returns
///
/// A vector of vectors, where each inner vector represents a grouped block of tokens.
pub fn group_lines_to_blocks(mut tokenized_lines: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
    let mut blocks: Vec<Vec<Token>> = Vec::new();
    let mut current_block: Vec<Token> = Vec::new();
    let mut previous_block: Vec<Token>;
    let lines = tokenized_lines.iter_mut();
    let mut is_inside_code_block = false;
    for line in lines {
        previous_block = blocks.last().unwrap_or(&Vec::new()).to_vec();

        // Appending all tokens between two code fences to one block
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
                if let Some(previous_line_start) = previous_block.first() {
                    match previous_line_start {
                        Token::Punctuation(string)
                            if string == "-"
                                && previous_block.get(1) == Some(&Token::Whitespace) =>
                        {
                            // Then it is either the start of a list or part of a list

                            previous_block.push(Token::Newline);
                            previous_block.extend(line.to_owned());
                            blocks.pop();
                            blocks.push(previous_block.clone());
                        }
                        Token::Punctuation(string) if string == "#" => {
                            blocks.push(line.to_owned());
                        }
                        _ => {
                            if line.len() > 1 {
                                current_block.extend(line.to_owned());
                            } else {
                                // Then this is a Setext heading 2
                                previous_block.insert(0, Token::Punctuation(String::from("#")));
                                previous_block.insert(1, Token::Punctuation(String::from("#")));
                                previous_block.insert(2, Token::Whitespace);
                                blocks.pop();
                                blocks.push(previous_block.clone());
                            }
                        }
                    }
                } else {
                    current_block.extend(line.to_owned());
                }
            }
            Some(Token::Tab) => {
                if line.len() > 1 {
                    let mut has_content: bool = false;
                    for idx in 1..line.len() {
                        match line.get(idx) {
                            Some(Token::Tab) | Some(Token::Whitespace) => continue,
                            None => {}
                            _ => {
                                has_content = true;
                                break;
                            }
                        }
                    }

                    if has_content {
                        // If there is content after the tab, then we append it to the previous
                        // block
                        if !previous_block.is_empty() {
                            let previous_line_start = previous_block.first();
                            match previous_line_start {
                                Some(Token::Punctuation(string))
                                    if string == "-"
                                        && previous_block.get(1) == Some(&Token::Whitespace) =>
                                {
                                    // If the previous block is a list, then we append the line to it
                                    previous_block.push(Token::Newline);
                                    previous_block.extend(line.to_owned());
                                    blocks.pop();
                                    blocks.push(previous_block.clone());
                                }
                                Some(Token::OrderedListMarker(_))
                                    if previous_block.get(1) == Some(&Token::Whitespace) =>
                                {
                                    // If the previous block is an ordered list, then we append the
                                    // line to it
                                    previous_block.push(Token::Newline);
                                    previous_block.extend(line.to_owned());
                                    blocks.pop();
                                    blocks.push(previous_block.clone());
                                }
                                _ => {
                                    // If the previous block is not a list, then we just add the
                                    // line to the current block
                                    current_block.extend(line.to_owned());
                                }
                            }
                        } else {
                            // If the previous block is empty, then we just add the line to the
                            // current block
                            current_block.extend(line.to_owned());
                        }
                    }
                }
            }
            Some(Token::OrderedListMarker(_)) => {
                if let Some(previous_line_start) = previous_block.first() {
                    match previous_line_start {
                        Token::OrderedListMarker(_)
                            if previous_block.get(1) == Some(&Token::Whitespace) =>
                        {
                            // If the previous block is a list, then we append the line to it
                            previous_block.push(Token::Newline);
                            previous_block.extend(line.to_owned());
                            blocks.pop();
                            blocks.push(previous_block.clone());
                        }
                        _ => {
                            current_block.extend(line.to_owned());
                        }
                    }
                } else {
                    current_block.extend(line.to_owned());
                }
            }
            Some(Token::ThematicBreak) => {
                // Check if the previous line starts with anything other than a heading
                // If so, then this is actually a setext heading 2
                if let Some(previous_line_start) = previous_block.first() {
                    match previous_line_start {
                        Token::Punctuation(string) if string == "#" => {
                            blocks.push(line.to_owned());
                        }
                        Token::Newline => blocks.push(line.to_owned()),
                        _ => {
                            previous_block.insert(0, Token::Punctuation(String::from("#")));
                            previous_block.insert(1, Token::Punctuation(String::from("#")));
                            previous_block.insert(2, Token::Whitespace);
                            blocks.pop();
                            blocks.push(previous_block.clone());
                        }
                    }
                } else {
                    current_block.extend(line.to_owned());
                }
            }
            Some(Token::CodeTick) => {
                current_block.extend(line.to_owned());
            }
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
