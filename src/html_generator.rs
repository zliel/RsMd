use crate::CONFIG;
use crate::types::{MdBlockElement, ToHtml};

pub fn generate_html(md_elements: Vec<MdBlockElement>) -> String {
    let mut html_output = String::new();

    // Build the HTML head
    let mut head = String::from(
        r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Markdown Document</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
    "#,
    );

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
