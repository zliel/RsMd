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
    html_rel_path: &str,
) -> String {
    let mut html_output = String::new();

    let head = generate_head(file_name, html_rel_path);

    let mut body = String::from("<body>\n");
    body.push_str(&generate_navbar(html_rel_path));
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

    let head = generate_head("index", "index.html");

    let mut body = String::from("<body>\n");
    body.push_str(&generate_navbar("index.html"));
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
fn generate_head(file_name: &str, html_rel_path: &str) -> String {
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
    let html_path = Path::new(html_rel_path);
    let depth = html_path.parent().map_or(0, |p| p.components().count());
    let mut css_path = PathBuf::new();
    for _ in 0..depth {
        css_path.push("..");
    }
    css_path.push("styles.css");
    let css_href = css_path.to_string_lossy();
    if css_file == "default" {
        head.push_str(format!("<link rel=\"stylesheet\" href=\"{}\">\n", css_href).as_str());
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
fn generate_navbar(html_rel_path: &str) -> String {
    let mut navbar = String::from("<header><nav>\n<ul>\n");

    let html_path = Path::new(html_rel_path);
    let depth = html_path.parent().map_or(0, |p| p.components().count());
    let mut home_path = PathBuf::new();
    for _ in 0..depth {
        home_path.push("..");
    }

    home_path.push("index.html");
    let home_href = home_path.to_string_lossy();

    navbar.push_str(format!("<li><a href=\"{}\">Home</a></li>\n", home_href).as_str());
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

/// Generates a default CSS stylesheet as a string.
pub fn generate_default_css() -> String {
    r#"
    body {
    background-color: #121212;
    color: #e0e0e0;
    font-family:
        -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu,
        Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    line-height: 1.75;
    margin: 0;
    padding: 0;
    }

    /* Card-like container for the page content */
    #content {
    background-color: #1e1e1e;
    max-width: 780px;
    margin: 1.5rem auto;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 0 0 1px #2c2c2c;
    }

    header {
    background-color: #1a1a1a;
    border-bottom: 1px solid #333;
    position: sticky;
    top: 0;
    z-index: 1000;
    }

    nav {
    padding: 1rem 2rem;
    display: flex;
    justify-content: flex-start;
    }

    nav ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: 1rem;
    }

    nav ul li {
    margin: 0;
    }

    nav ul li a {
    color: #ddd;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    transition: background-color 0.2s ease, color 0.2s ease;
    }

    nav ul li a:hover {
    background-color: #2f2f2f;
    color: #fff;
    }

    nav ul li a.active {
    background-color: #4ea1f3;
    color: #121212;
    }
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
    color: #ffffff;
    line-height: 1.3;
    margin-top: 2rem;
    margin-bottom: 1rem;
    }

    h1 {
    font-size: 2.25rem;
    border-bottom: 2px solid #2c2c2c;
    padding-bottom: 0.3rem;
    }
    h2 {
    font-size: 1.75rem;
    border-bottom: 1px solid #2c2c2c;
    padding-bottom: 0.2rem;
    }
    h3 {
    font-size: 1.5rem;
    }
    h4 {
    font-size: 1.25rem;
    }
    h5,
    h6 {
    font-size: 1rem;
    font-weight: normal;
    }

    p {
    margin-bottom: 1.2rem;
    }

    a {
    color: #4ea1f3;
    text-decoration: none;
    transition: color 0.2s ease-in-out;
    }
    a:hover {
    color: #82cfff;
    text-decoration: underline;
    }

    img {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1.5rem auto;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    }

    pre {
    background-color: #2a2a2a;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    font-size: 0.9rem;
    box-shadow: inset 0 0 0 1px #333;
    }
    code {
    font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    background-color: #2a2a2a;
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-size: 0.95em;
    color: #dcdcdc;
    }

    blockquote {
    border-left: 4px solid #555;
    padding-left: 1rem;
    color: #aaa;
    font-style: italic;
    margin: 1.5rem 0;
    background-color: #1a1a1a;
    border-radius: 4px;
    }

    ul,
    ol {
    padding-left: 1.5rem;
    margin-bottom: 1.2rem;
    }
    li {
    margin-bottom: 0.5rem;
    }

    table {
    width: 100%;
    border-collapse: collapse;
    margin: 2rem 0;
    background-color: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
    font-size: 0.95rem;
    }

    th,
    td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid #333;
    }

    th {
    background-color: #2a2a2a;
    color: #ffffff;
    font-weight: 600;
    }

    tr:nth-child(even) td {
    background-color: #222;
    }

    tr:hover td {
    background-color: #2f2f2f;
    }

    td {
    color: #ddd;
    }

    hr {
    border: none;
    border-top: 1px solid #333;
    margin: 2rem 0;
    }
    "#
    .to_string()
}
