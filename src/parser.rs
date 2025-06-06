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
        items: Vec<MdBlockElement>,
    },
    HorizontalRule,
}

pub enum MdInlineElement {
    Text { content: String },
    Bold { content: Vec<MdInlineElement> },
    Italic { content: String },
    Link { text: String, url: String }, // Look into if this is a good or bad way to do this
}
