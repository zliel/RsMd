#[derive(Debug)]
pub enum MdBlockElement {
    Header {
        level: u8,
        content: Vec<MdInlineElement>,
    },
    Paragraph {
        content: Vec<MdInlineElement>,
    },
    CodeBlock {
        lines: Vec<String>,
    },
    UnorderedList {
        items: Vec<MdListItem>,
    },
    HorizontalRule,
}

#[derive(Debug)]
pub struct MdListItem {
    content: Vec<MdBlockElement>,
}

#[derive(Debug)]
pub enum MdInlineElement {
    Text {
        content: String,
    },
    Bold {
        content: Vec<MdInlineElement>,
    },
    Italic {
        content: Vec<MdInlineElement>,
    },
    Link {
        text: Vec<MdInlineElement>,
        url: String,
    },
}

