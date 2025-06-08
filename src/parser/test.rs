use crate::parser::{MdBlockElement::*, MdInlineElement::*, *};

mod inline {
    use super::*;

    #[test]
    fn text() {
        assert_eq!(
            parse_inline("Plain text."),
            vec![Text {
                content: String::from("Plain text.")
            }]
        );
    }

    #[test]
    fn bold() {
        assert_eq!(
            parse_inline("**Bold** text"),
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
            parse_inline("*Italic* text"),
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
    fn emphasis() {
        assert_eq!(
            parse_inline("This is **bold** and *italic* text."),
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
    fn link() {
        assert_eq!(
            parse_inline("[link text](http://example.com)"),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text")
                }],
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_emphasis() {
        assert_eq!(
            parse_inline("[**bold link text**](http://example.com)"),
            vec![Link {
                text: vec![Bold {
                    content: vec![Text {
                        content: String::from("**bold link text**")
                    }]
                }],
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_internal_hashes() {
        assert_eq!(
            parse_inline("[link text with #hash](http://example.com)"),
            vec![Link {
                text: vec![Text {
                    content: String::from("link text with #hash")
                }],
                url: String::from("http://example.com")
            }]
        );
    }

    #[test]
    fn link_with_mixed_emphasis() {
        assert_eq!(
            parse_inline("[*italic, **bold and italic***](http://example.com)"),
            vec![Link {
                text: vec![
                    Italic {
                        content: vec![Text {
                            content: String::from("italic, ")
                        }],
                    },
                    Bold {
                        content: vec![Text {
                            content: String::from("bold and italic")
                        }]
                    }
                ],
                url: String::from("http://example.com")
            }]
        );
    }
}

#[test]
    assert_eq!(
        }]
    );
}

#[test]
    assert_eq!(
        }]
    );
}

#[test]
    assert_eq!(
        }]
    );
}

#[test]
    assert_eq!(
                },
                Bold {
                    content: vec![Text {
                    }]
                }
        }]
}
