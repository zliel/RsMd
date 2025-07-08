//! This module provides functionality to generate HTML from markdown block elements.
use crate::CONFIG;
use crate::types::{MdBlockElement, ToHtml};

/// Generates an HTML string from a vector of MdBlockElements
///
/// # Arguments
/// * `md_elements` - A vector of `MdBlockElement` instances representing the markdown content.
///
/// # Returns
/// Returns a `String` containing the generated HTML.
pub fn generate_html(file_name: &str, md_elements: Vec<MdBlockElement>) -> String {
    let mut html_output = String::new();

    // Build the HTML head
    let mut head = String::from(
        r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
    "#,
    );

    // Remove the file extension from the file name and make it title case
    let title = format_title(file_name);
    head.push_str(&format!("<title>{}</title>\n", title));

    let css_file = CONFIG.get().unwrap().html.css_file.clone();
    if css_file == "default" {
        head.push_str("<link rel=\"stylesheet\" href=\"styles.css\">\n");
    } else {
        head.push_str(&format!(
            "<link rel=\"stylesheet\" href=\"{}\">\n",
            css_file
        ));
    }

    head.push_str("</head>\n");

    let mut body = String::from("<body>\n");
    let inner_html: String = md_elements
        .iter()
        .map(|element| element.to_html())
        .collect::<Vec<String>>()
        .join("\n");

    body.push_str(&inner_html);
    body.push_str("\n</body>\n");

    html_output.push_str(&head);
    html_output.push_str(&body);
    html_output.push_str("</html>\n");

    html_output
}

/// Formats the file name to create a title for the HTML document
///
/// # Arguments
/// * `file_name` - The name of the file, typically ending with `.md`.
///
/// # Returns
/// The formatted title (i.e. "my_test_page.md" -> "My Test Page")
fn format_title(file_name: &str) -> String {
    let title = file_name.trim_end_matches(".md").replace('_', " ");

    title
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
