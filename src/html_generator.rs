use crate::types::{MdBlockElement, ToHtml};

pub fn generate_html(md_elements: Vec<MdBlockElement>) -> String {
    let mut html_output = String::new();

    // Build the HTML head
    let head = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Markdown Document</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
    </head>
    "#;

    let mut body = String::from("<body>\n");
    let inner_html: String = md_elements
        .iter()
        .map(|element| element.to_html())
        .collect::<Vec<String>>()
        .join("\n");

    body.push_str(&inner_html);
    body.push_str("\n</body>\n");

    html_output.push_str(head);
    html_output.push_str(&body);
    html_output.push_str("</html>\n");

    html_output
}
