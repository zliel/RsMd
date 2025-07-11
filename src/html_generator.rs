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
pub fn generate_html(
    file_name: &str,
    md_elements: Vec<MdBlockElement>,
    output_dir: &str,
    input_dir: &str,
) -> String {
    let mut html_output = String::new();

    let head = generate_head(file_name);

    let mut body = String::from("<body>\n");
    body.push_str(&generate_navbar());
    body.push_str("<div id=\"content\">\n");

    let inner_html: String = md_elements
        .iter()
        .map(|element| element.to_html(output_dir, input_dir))
        .collect::<Vec<String>>()
        .join("\n");

    body.push_str(&inner_html);
    body.push_str("\n</div>\n</body>\n");

    html_output.push_str(&head);
    html_output.push_str(&body);
    html_output.push_str("</html>\n");

    html_output
}

/// Generates the index HTML file that lists all pages
///
/// # Arguments
/// * `file_names` - A slice of `String` containing the names of the markdown files.
///
/// # Returns
/// Returns a `String` containing the generated HTML for the index page.
pub fn generate_index(file_names: &[String]) -> String {
    let mut html_output = String::new();

    let head = generate_head("index");

    let mut body = String::from("<body>\n");
    body.push_str(&generate_navbar());
    body.push_str("<div id=\"content\">\n");
    body.push_str("<h1>All Pages</h1>\n");

    file_names.iter().for_each(|file_name| {
        body.push_str(&format!(
            "<a href=\".\\{}.html\">{}</a><br>\n",
            file_name.trim_end_matches(".md"),
            format_title(file_name)
        ));
    });

    body.push_str("\n</div>\n</body>\n");

    html_output.push_str(&head);
    html_output.push_str(&body);

    html_output
}

/// Generates the HTML head section
///
/// # Arguments
/// * `file_name` - The name of the markdown file, used to set the title of the HTML document.
fn generate_head(file_name: &str) -> String {
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

    let favicon_file = CONFIG.get().unwrap().html.favicon_file.clone();
    if !favicon_file.is_empty() {
        let favicon_file = favicon_file.rsplit("/").next().unwrap_or(&favicon_file);

        head.push_str(&format!(
            "<link rel=\"icon\" href=\"media/{}\">\n",
            favicon_file
        ));
    }

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
    head
}

/// Generates the HTML for the navigation bar
fn generate_navbar() -> String {
    let mut navbar = String::from("<header><nav>\n<ul>\n");

    navbar.push_str("<li><a href=\"index.html\">Home</a></li>\n");
    navbar.push_str("</ul>\n</nav>\n</header>\n");
    navbar
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
