use crate::lexer::tokenize;
use crate::parser::{parse_block, parse_inline};
use crate::types::{MdBlockElement::*, MdInlineElement::*, *};

mod inline {
    use super::*;

    #[test]
    fn text() {
        assert_eq!(
            parse_inline(tokenize("Plain text.")),
            vec![Text {
                content: String::from("Plain text.")
            }]
        );
    }

    #[test]
    fn escape_char() {
        assert_eq!(
            parse_inline(tokenize("\\*escaped char\\*")),
            vec![Text {
                content: String::from("\\*escaped char\\*")
            }]
        );
    }

    #[test]
    fn bold() {
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
        assert_eq!(
            parse_inline(tokenize("[link text](http://example.com)")),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text")
                }],
                title: Some(String::from("")),
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_emphasis() {
        assert_eq!(
            parse_inline(tokenize("[**bold link text**](http://example.com)")),
            vec![Link {
                text: vec![Bold {
                    content: vec![Text {
                        content: String::from("bold link text")
                    }]
                }],
                title: Some(String::from("")),
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_internal_hashes() {
        assert_eq!(
            parse_inline(tokenize("[link text with #hash](http://example.com)")),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text with #hash")
                }],
                title: Some(String::from("")),
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_mixed_emphasis() {
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
                title: Some(String::from("")),
                url: String::from("http://example.com")
            }]
        );
    }
}

mod block {
    use super::*;

    #[test]
    fn test_heading() {
        println!("{:?}", tokenize("# Heading 1"));
        assert_eq!(
            parse_block(tokenize("# Heading 1")),
            Header {
                level: 1,
                content: vec![Text {
                    content: String::from("Heading 1")
                }]
            }
        );
    }

    #[test]
    fn test_multilevel_heading() {
        assert_eq!(
            parse_block(tokenize("### Heading 3")),
            Header {
                level: 3,
                content: vec![Text {
                    content: String::from("Heading 3")
                }]
            }
        );
    }

    #[test]
    fn test_heading_with_internal_hashes() {
        assert_eq!(
            parse_block(tokenize("## Heading 2 with #internal #hashes")),
            Header {
                level: 2,
                content: vec![Text {
                    content: String::from("Heading 2 with #internal #hashes")
                }]
            }
        );
    }

    #[test]
    fn test_heading_with_emphases() {
        assert_eq!(
            parse_block(tokenize("## Heading 2 with **bold words**")),
            Header {
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
            }
        )
    }
}
