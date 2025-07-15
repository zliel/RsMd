use crate::config::init_config;
use crate::lexer::tokenize;
use crate::parser::{parse_block, parse_inline};
use crate::types::{MdBlockElement::*, MdInlineElement::*, MdListItem, ToHtml};

use std::sync::Once;
static INIT: Once = Once::new();

fn init_test_config() {
    INIT.call_once(|| {
        init_config("config.toml").expect("Failed to initialize test config");
    });
}

mod inline {
    use super::*;

    #[test]
    fn text() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("Plain text.")),
            vec![Text {
                content: String::from("Plain text.")
            }]
        );
    }

    #[test]
    fn escape_char() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("\\*escaped char\\*")),
            vec![Text {
                content: String::from("\\*escaped char\\*")
            }]
        );
    }

    #[test]
    fn bold() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("**Bold** text")),
            vec![
                Bold {
                    content: vec![Text {
                        content: String::from("Bold")
                    }]
                },
                Text {
                    content: String::from(" text")
                }
            ]
        );
    }

    #[test]
    fn italic() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("*Italic* text")),
            vec![
                Italic {
                    content: vec![Text {
                        content: String::from("Italic")
                    }]
                },
                Text {
                    content: String::from(" text")
                }
            ]
        );
    }

    #[test]
    fn multiple_emphasis() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("This is **bold** and *italic* text.")),
            vec![
                Text {
                    content: String::from("This is ")
                },
                Bold {
                    content: vec![Text {
                        content: String::from("bold")
                    }]
                },
                Text {
                    content: String::from(" and ")
                },
                Italic {
                    content: vec![Text {
                        content: String::from("italic")
                    }]
                },
                Text {
                    content: String::from(" text.")
                }
            ]
        )
    }

    #[test]
    fn mixed_emphasis() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("**_Bold and italic._**")),
            vec![Bold {
                content: vec![Italic {
                    content: vec![Text {
                        content: String::from("Bold and italic.")
                    }]
                }]
            }]
        )
    }

    #[test]
    fn mixed_emphasis_separated() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("_Italic **and bold**_")),
            vec![Italic {
                content: vec![
                    Text {
                        content: String::from("Italic ")
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("and bold")
                        }]
                    }
                ]
            }]
        )
    }

    #[test]
    fn link() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("[link text](http://example.com)")),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text")
                }],
                title: None,
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_emphasis() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("[**bold link text**](http://example.com)")),
            vec![Link {
                text: vec![Bold {
                    content: vec![Text {
                        content: String::from("bold link text")
                    }]
                }],
                title: None,
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_internal_hashes() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("[link text with #hash](http://example.com)")),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text with #hash")
                }],
                title: None,
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_mixed_emphasis() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize(
                "[_italic, **bold and italic**_](http://example.com)"
            )),
            vec![Link {
                text: vec![Italic {
                    content: vec![
                        Text {
                            content: String::from("italic, ")
                        },
                        Bold {
                            content: vec![Text {
                                content: String::from("bold and italic")
                            }]
                        }
                    ]
                }],
                title: None,
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_title() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("[link text](http://example.com \"Title\")")),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text")
                }],
                title: Some(String::from("Title")),
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_emphasized_title() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize(
                "[**bold link text**](http://example.com \"Title with **bold**\")"
            )),
            vec![Link {
                text: vec![Bold {
                    content: vec![Text {
                        content: String::from("bold link text")
                    }]
                }],
                title: Some(String::from("Title with **bold**")),
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn image() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("![alt text](http://example.com/image.png)")),
            vec![Image {
                alt_text: String::from("alt text"),
                title: None,
                url: String::from("http://example.com/image.png")
            }]
        );
    }

    #[test]
    fn image_with_title() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize(
                "![alt text](http://example.com/image.png \"Title\")"
            )),
            vec![Image {
                alt_text: String::from("alt text"),
                title: Some(String::from("Title")),
                url: String::from("http://example.com/image.png")
            }]
        );
    }

    #[test]
    fn image_with_empty_alt_text() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize("![](http://example.com/image.png)")),
            vec![Image {
                alt_text: String::from(""),
                title: None,
                url: String::from("http://example.com/image.png")
            }]
        )
    }

    #[test]
    fn image_with_emphasized_alt_text() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize(
                "![**bold alt text**](http://example.com/image.png)"
            )),
            vec![Image {
                alt_text: String::from("bold alt text"), //Only plain string content is added
                title: None,
                url: String::from("http://example.com/image.png")
            }]
        );
    }

    #[test]
    fn image_with_emphasized_title() {
        init_test_config();
        assert_eq!(
            parse_inline(tokenize(
                "![alt text](http://example.com/image.png \"**bold title**\")"
            )),
            vec![Image {
                alt_text: String::from("alt text"),
                title: Some(String::from("**bold title**")),
                url: String::from("http://example.com/image.png")
            }]
        );
    }
}

mod block {
    use crate::{
        parser::{group_lines_to_blocks, parse_blocks},
        types::{MdTableCell, TableAlignment},
    };

    use super::*;

    #[test]
    fn heading() {
        init_test_config();
        println!("{:?}", tokenize("# Heading 1"));
        assert_eq!(
            parse_block(tokenize("# Heading 1")),
            Some(Header {
                level: 1,
                content: vec![Text {
                    content: String::from("Heading 1")
                }]
            })
        );
    }

    #[test]
    fn multilevel_heading() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("### Heading 3")),
            Some(Header {
                level: 3,
                content: vec![Text {
                    content: String::from("Heading 3")
                }]
            })
        );
    }

    #[test]
    fn heading_with_internal_hashes() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("## Heading 2 with #internal #hashes")),
            Some(Header {
                level: 2,
                content: vec![Text {
                    content: String::from("Heading 2 with #internal #hashes")
                }]
            })
        );
    }

    #[test]
    fn heading_with_emphases() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("## Heading 2 with **bold words**")),
            Some(Header {
                level: 2,
                content: vec![
                    Text {
                        content: String::from("Heading 2 with ")
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("bold words")
                        }]
                    }
                ]
            })
        )
    }

    #[test]
    fn paragraph() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("This is a paragraph.")),
            Some(Paragraph {
                content: vec![Text {
                    content: String::from("This is a paragraph.")
                }]
            })
        );
    }

    #[test]
    fn multiple_paragraphs() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("First paragraph."),
                tokenize("Second paragraph.")
            ])),
            vec![Paragraph {
                content: vec![Text {
                    content: String::from("First paragraph. Second paragraph.")
                }]
            }]
        );
    }

    #[test]
    fn multiline_paragraphs() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("First line.\nSecond line.")),
            Some(Paragraph {
                content: vec![
                    Text {
                        content: String::from("First line.")
                    },
                    Text {
                        content: String::from("Second line.")
                    }
                ]
            })
        );
    }

    #[test]
    fn paragraph_with_emphasis() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("This is a paragraph with **bold text**.")),
            Some(Paragraph {
                content: vec![
                    Text {
                        content: String::from("This is a paragraph with ")
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("bold text")
                        }]
                    },
                    Text {
                        content: String::from(".")
                    }
                ]
            })
        );
    }

    #[test]
    fn paragraph_with_mixed_emphasis() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize(
                "This is a paragraph with **bold text** and *italic text*."
            )),
            Some(Paragraph {
                content: vec![
                    Text {
                        content: String::from("This is a paragraph with ")
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("bold text")
                        }]
                    },
                    Text {
                        content: String::from(" and ")
                    },
                    Italic {
                        content: vec![Text {
                            content: String::from("italic text")
                        }]
                    },
                    Text {
                        content: String::from(".")
                    }
                ]
            })
        );
    }

    #[test]
    fn paragraph_with_link() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize(
                "This is a paragraph with [a link](http://example.com)."
            )),
            Some(Paragraph {
                content: vec![
                    Text {
                        content: String::from("This is a paragraph with ")
                    },
                    Link {
                        text: vec![Text {
                            content: String::from("a link")
                        }],
                        title: None,
                        url: String::from("http://example.com")
                    },
                    Text {
                        content: String::from(".")
                    }
                ]
            })
        );
    }

    #[test]
    fn paragraph_with_image_and_emphasis() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize(
                "This is a paragraph with ![an image](http://example.com/image.png) and **bold text**."
            )),
            Some(Paragraph {
                content: vec![
                    Text {
                        content: String::from("This is a paragraph with ")
                    },
                    Image {
                        alt_text: String::from("an image"),
                        title: None,
                        url: String::from("http://example.com/image.png")
                    },
                    Text {
                        content: String::from(" and ")
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("bold text")
                        }]
                    },
                    Text {
                        content: String::from(".")
                    }
                ]
            })
        );
    }

    #[test]
    fn complex_paragraph() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize(
                    "This is a paragraph with **bold text**, *italic text*, and [a link](http://example.com)."
                ),
                tokenize(
                    "It also contains ![an image](http://example.com/image.png) and some `inline code`."
                ),
                tokenize("It also contains a `code span`, and the following code block:"),
                tokenize("```rust"),
                tokenize("fn main() {"),
                tokenize("    println!(\"Hello, world!\");"),
                tokenize("}"),
                tokenize("```")
            ])),
            vec![
                Paragraph {
                    content: vec![
                        Text {
                            content: String::from("This is a paragraph with ")
                        },
                        Bold {
                            content: vec![Text {
                                content: String::from("bold text")
                            }]
                        },
                        Text {
                            content: String::from(", ")
                        },
                        Italic {
                            content: vec![Text {
                                content: String::from("italic text")
                            }]
                        },
                        Text {
                            content: String::from(", and ")
                        },
                        Link {
                            text: vec![Text {
                                content: String::from("a link")
                            }],
                            title: None,
                            url: String::from("http://example.com")
                        },
                        Text {
                            content: String::from(". It also contains ")
                        },
                        Image {
                            alt_text: String::from("an image"),
                            title: None,
                            url: String::from("http://example.com/image.png")
                        },
                        Text {
                            content: String::from(" and some ")
                        },
                        Code {
                            content: String::from("inline code")
                        },
                        Text {
                            content: String::from(". It also contains a ")
                        },
                        Code {
                            content: String::from("code span")
                        },
                        Text {
                            content: String::from(", and the following code block:")
                        }
                    ]
                },
                CodeBlock {
                    language: Some(String::from("rust")),
                    lines: vec![String::from(
                        "fn main() {\nprintln!(\"Hello, world!\");\n}\n"
                    )]
                }
            ]
        )
    }

    #[test]
    fn unordered_list() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("- Item 1"),
                tokenize("- Item 2")
            ])),
            vec![UnorderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 1")
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 2")
                            }]
                        }
                    }
                ]
            }]
        );
    }

    #[test]
    fn unordered_list_with_nested_items() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("- Item 1"),
                tokenize("    - Nested Item 1.1"),
                tokenize("    - Nested Item 1.2"),
                tokenize("- Item 2")
            ])),
            vec![UnorderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 1")
                            }]
                        }
                    },
                    MdListItem {
                        content: UnorderedList {
                            items: vec![
                                MdListItem {
                                    content: Paragraph {
                                        content: vec![Text {
                                            content: String::from("Nested Item 1.1")
                                        }]
                                    }
                                },
                                MdListItem {
                                    content: Paragraph {
                                        content: vec![Text {
                                            content: String::from("Nested Item 1.2")
                                        }]
                                    }
                                }
                            ]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 2")
                            }]
                        }
                    }
                ]
            }]
        );
    }

    #[test]
    fn unordered_list_with_inlines() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("1. **Bold Item 1**"),
                tokenize("2. *Italic Item 2*"),
                tokenize("3. [Link Item 3](http://example.com)"),
                tokenize("4. ![Image Item 4](http://example.com/image.png)"),
            ])),
            vec![OrderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Bold {
                                content: vec![Text {
                                    content: String::from("Bold Item 1")
                                }]
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Italic {
                                content: vec![Text {
                                    content: String::from("Italic Item 2")
                                }]
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Link {
                                text: vec![Text {
                                    content: String::from("Link Item 3")
                                }],
                                title: None,
                                url: String::from("http://example.com")
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Image {
                                alt_text: String::from("Image Item 4"),
                                title: None,
                                url: String::from("http://example.com/image.png")
                            }]
                        }
                    }
                ]
            }]
        )
    }

    #[test]
    fn ordered_list() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("1. First"),
                tokenize("2. Second")
            ])),
            vec![OrderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("First")
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Second")
                            }]
                        }
                    }
                ]
            }]
        );
    }

    #[test]
    fn ordered_list_with_nested_items() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("1. Item 1"),
                tokenize("    1. Nested Item 1.1"),
                tokenize("    2. Nested Item 1.2"),
                tokenize("2. Item 2")
            ])),
            vec![OrderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 1")
                            }]
                        }
                    },
                    MdListItem {
                        content: OrderedList {
                            items: vec![
                                MdListItem {
                                    content: Paragraph {
                                        content: vec![Text {
                                            content: String::from("Nested Item 1.1")
                                        }]
                                    }
                                },
                                MdListItem {
                                    content: Paragraph {
                                        content: vec![Text {
                                            content: String::from("Nested Item 1.2")
                                        }]
                                    }
                                }
                            ]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Text {
                                content: String::from("Item 2")
                            }]
                        }
                    }
                ]
            }]
        );
    }

    #[test]
    fn ordered_list_with_inlines() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("1. **Bold Item 1**"),
                tokenize("2. *Italic Item 2*"),
                tokenize("3. [Link Item 3](http://example.com)"),
                tokenize("4. ![Image Item 4](http://example.com/image.png \"Some title\")"),
            ])),
            vec![OrderedList {
                items: vec![
                    MdListItem {
                        content: Paragraph {
                            content: vec![Bold {
                                content: vec![Text {
                                    content: String::from("Bold Item 1")
                                }]
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Italic {
                                content: vec![Text {
                                    content: String::from("Italic Item 2")
                                }]
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Link {
                                text: vec![Text {
                                    content: String::from("Link Item 3")
                                }],
                                title: None,
                                url: String::from("http://example.com")
                            }]
                        }
                    },
                    MdListItem {
                        content: Paragraph {
                            content: vec![Image {
                                alt_text: String::from("Image Item 4"),
                                title: Some(String::from("Some title")),
                                url: String::from("http://example.com/image.png")
                            }]
                        }
                    }
                ]
            }]
        )
    }

    #[test]
    fn code_block() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("```\ncode block\n```")),
            Some(CodeBlock {
                language: None,
                lines: vec![String::from("code block\n")]
            })
        );
    }

    #[test]
    fn fenced_code_block_with_language() {
        init_test_config();
        assert_eq!(
            parse_block(tokenize("```rust\nfn main() {}\n```")),
            Some(CodeBlock {
                language: Some(String::from("rust")),
                lines: vec![String::from("\nfn main() {}\n")]
            })
        );
    }

    #[test]
    fn table_all_left_align() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("| Header 1 | Header 2 |"),
                tokenize("| :-- | :-- |"),
                tokenize("| Cell 1 | Cell 2 |"),
                tokenize("| Cell 3 | Cell 4 |")
            ])),
            vec![Table {
                headers: vec![
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 1 ")
                        }],
                        alignment: TableAlignment::Left,
                        is_header: true,
                    },
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 2 ")
                        }],
                        alignment: TableAlignment::Left,
                        is_header: true,
                    }
                ],
                body: vec![
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 1 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 2 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        }
                    ],
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 3 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 4 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        }
                    ]
                ]
            }]
        );
    }

    #[test]
    fn table_mixed_align() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("| Header 1 | Header 2 | Header 3 |"),
                tokenize("| :-- | :-: | --: |"),
                tokenize("| Cell 1 | Cell 2 | Cell 3 |"),
                tokenize("| Cell 4 | Cell 5 | Cell 6 |")
            ])),
            vec![Table {
                headers: vec![
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 1 ")
                        }],
                        alignment: TableAlignment::Left,
                        is_header: true,
                    },
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 2 ")
                        }],
                        alignment: TableAlignment::Center,
                        is_header: true,
                    },
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 3 ")
                        }],
                        alignment: TableAlignment::Right,
                        is_header: true,
                    }
                ],
                body: vec![
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 1 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 2 ")
                            }],
                            alignment: TableAlignment::Center,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 3 ")
                            }],
                            alignment: TableAlignment::Right,
                            is_header: false,
                        }
                    ],
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 4 ")
                            }],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 5 ")
                            }],
                            alignment: TableAlignment::Center,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 6 ")
                            }],
                            alignment: TableAlignment::Right,
                            is_header: false,
                        }
                    ]
                ]
            }]
        );
    }

    #[test]
    fn table_no_align() {
        init_test_config();

        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("| Header 1 | Header 2 |"),
                tokenize("| -- | -- |"),
                tokenize("| Cell 1 | Cell 2 |"),
                tokenize("| Cell 3 | Cell 4 |")
            ])),
            vec![Table {
                headers: vec![
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 1 ")
                        }],
                        alignment: TableAlignment::None,
                        is_header: true,
                    },
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 2 ")
                        }],
                        alignment: TableAlignment::None,
                        is_header: true,
                    }
                ],
                body: vec![
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 1 ")
                            }],
                            alignment: TableAlignment::None,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 2 ")
                            }],
                            alignment: TableAlignment::None,
                            is_header: false,
                        }
                    ],
                    vec![
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 3 ")
                            }],
                            alignment: TableAlignment::None,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![Text {
                                content: String::from(" Cell 4 ")
                            }],
                            alignment: TableAlignment::None,
                            is_header: false,
                        }
                    ]
                ]
            }]
        );
    }

    #[test]
    fn table_with_inline_content() {
        init_test_config();
        assert_eq!(
            parse_blocks(group_lines_to_blocks(vec![
                tokenize("| Header 1 | Header 2 |"),
                tokenize("| :-- | :-- |"),
                tokenize("| **Bold Cell** | *Italic Cell* |"),
                tokenize("| [Link](http://example.com) | ![Image](http://example.com/image.png) |")
            ])),
            vec![Table {
                headers: vec![
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 1 ")
                        }],
                        alignment: TableAlignment::Left,
                        is_header: true,
                    },
                    MdTableCell {
                        content: vec![Text {
                            content: String::from(" Header 2 ")
                        }],
                        alignment: TableAlignment::Left,
                        is_header: true,
                    }
                ],
                body: vec![
                    vec![
                        MdTableCell {
                            content: vec![
                                Text {
                                    content: " ".to_string()
                                },
                                Bold {
                                    content: vec![Text {
                                        content: String::from("Bold Cell")
                                    }]
                                },
                                Text {
                                    content: " ".to_string()
                                }
                            ],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![
                                Text {
                                    content: " ".to_string()
                                },
                                Italic {
                                    content: vec![Text {
                                        content: String::from("Italic Cell")
                                    }]
                                },
                                Text {
                                    content: " ".to_string()
                                }
                            ],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        }
                    ],
                    vec![
                        MdTableCell {
                            content: vec![
                                Text {
                                    content: " ".to_string()
                                },
                                Link {
                                    text: vec![Text {
                                        content: String::from("Link")
                                    }],
                                    title: None,
                                    url: String::from("http://example.com")
                                },
                                Text {
                                    content: " ".to_string()
                                }
                            ],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        },
                        MdTableCell {
                            content: vec![
                                Text {
                                    content: " ".to_string()
                                },
                                Image {
                                    alt_text: String::from("Image"),
                                    title: None,
                                    url: String::from("http://example.com/image.png")
                                },
                                Text {
                                    content: " ".to_string()
                                }
                            ],
                            alignment: TableAlignment::Left,
                            is_header: false,
                        }
                    ]
                ]
            }]
        );
    }
}

mod html_generation {
    use crate::parser::{group_lines_to_blocks, parse_blocks};

    use super::*;

    mod inline {
        use super::*;

        #[test]
        fn text() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("Plain text."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "Plain text."
            );
        }

        #[test]
        fn escape_char() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("\\*escaped chars work\\*"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "\\*escaped chars work\\*"
            );
        }

        #[test]
        fn bold() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("**Bold** text"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<b>Bold</b> text"
            );
        }

        #[test]
        fn italic() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("*Italic* text"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<i>Italic</i> text"
            );
        }

        #[test]
        fn mixed_emphasis() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("This is **bold** and *italic* text."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "This is <b>bold</b> and <i>italic</i> text."
            );
        }

        #[test]
        fn link() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("[link text](http://example.com)"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<a href=\"http://example.com\" target=\"_blank\">link text⮺</a>"
            );
        }

        #[test]
        fn image() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("![alt text](http://example.com/image.png)"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<img src=\"http://example.com/image.png\" alt=\"alt text\"/>"
            );
        }

        #[test]
        fn code_span() {
            init_test_config();
            assert_eq!(
                parse_inline(tokenize("This is `inline code`."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "This is <code>inline code</code>."
            );
        }
    }

    mod block {
        use super::*;

        #[test]
        fn plain_text_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("Plain text."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p>Plain text.</p>"
            );
        }

        #[test]
        fn bold_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("**Bold** text"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p><b>Bold</b> text</p>"
            );
        }

        #[test]
        fn italic_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("*Italic* text"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p><i>Italic</i> text</p>"
            );
        }

        #[test]
        fn mixed_emphasis_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("This is **bold** and *italic* text."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p>This is <b>bold</b> and <i>italic</i> text.</p>"
            );
        }

        #[test]
        fn link_in_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("[link text](http://example.com)"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p><a href=\"http://example.com\" target=\"_blank\">link text⮺</a></p>"
            );
        }

        #[test]
        fn image_in_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("![alt text](http://example.com/image.png)"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p><img src=\"http://example.com/image.png\" alt=\"alt text\"/></p>"
            );
        }

        #[test]
        fn code_span_in_paragraph() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("This is `inline code`."))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<p>This is <code>inline code</code>.</p>"
            );
        }

        #[test]
        fn heading() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("# Heading 1"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<h1>Heading 1</h1>"
            );
        }

        #[test]
        fn multilevel_heading() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("### Heading 3"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<h3>Heading 3</h3>"
            );
        }

        #[test]
        fn heading_with_emphasis() {
            init_test_config();
            assert_eq!(
                parse_block(tokenize("## Heading 2 with **bold words**"))
                    .iter()
                    .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                    .collect::<String>(),
                "<h2>Heading 2 with <b>bold words</b></h2>"
            );
        }

        #[test]
        fn code_block() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(
                    ["```\n", "code block", "second line", "```"]
                        .iter()
                        .map(|tokens| tokenize(tokens))
                        .collect::<Vec<_>>()
                ))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<pre><code>code block\nsecond line\n</code></pre>"
            );
        }

        #[test]
        fn code_block_with_language() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(
                    ["```rust", "fn main() {}", "```"]
                        .iter()
                        .map(|tokens| tokenize(tokens))
                        .collect::<Vec<_>>()
                ))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<pre><code class=\"language-rust\">fn main() {}\n</code></pre>"
            );
        }

        #[test]
        fn unordered_list() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(vec![
                    tokenize("- Item 1"),
                    tokenize("- Item 2")
                ]))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<ul><li><p>Item 1</p></li><li><p>Item 2</p></li></ul>" // Note that list items contain block-level elements, so text becomes paragraphs
            );
        }

        #[test]
        fn unordered_list_with_nested_items() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(vec![
                    tokenize("- Item 1"),
                    tokenize("    - Nested Item 1.1"),
                    tokenize("    - Nested Item 1.2"),
                    tokenize("- Item 2")
                ]))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<ul><li><p>Item 1</p></li><ul><li><p>Nested Item 1.1</p></li><li><p>Nested Item 1.2</p></li></ul><li><p>Item 2</p></li></ul>"
            );
        }

        #[test]
        fn ordered_list() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(vec![
                    tokenize("1. First"),
                    tokenize("2. Second")
                ]))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<ol><li><p>First</p></li><li><p>Second</p></li></ol>"
            );
        }

        #[test]
        fn ordered_list_with_nested_items() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(vec![
                    tokenize("1. Item 1"),
                    tokenize("    1. Nested Item 1.1"),
                    tokenize("    2. Nested Item 1.2"),
                    tokenize("2. Item 2")
                ]))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<ol><li><p>Item 1</p></li><ol><li><p>Nested Item 1.1</p></li><li><p>Nested Item 1.2</p></li></ol><li><p>Item 2</p></li></ol>"
            );
        }

        #[test]
        fn ordered_list_with_inlines() {
            init_test_config();
            assert_eq!(
                parse_blocks(group_lines_to_blocks(vec![
                    tokenize("1. **Bold Item 1**"),
                    tokenize("2. *Italic Item 2*"),
                    tokenize("3. [Link Item 3](http://example.com)"),
                    tokenize("4. ![Image Item 4](http://example.com/image.png \"Some title\")"),
                ]))
                .iter()
                .map(|el| el.to_html("test_output", "test_input", "test_rel_path"))
                .collect::<String>(),
                "<ol><li><p><b>Bold Item 1</b></p></li><li><p><i>Italic Item 2</i></p></li><li><p><a href=\"http://example.com\" target=\"_blank\">Link Item 3⮺</a></p></li><li><p><img src=\"http://example.com/image.png\" alt=\"Image Item 4\" title=\"Some title\"/></p></li></ol>"
            );
        }
    }
}
