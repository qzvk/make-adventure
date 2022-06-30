use super::{block::BlockKind, line::DirectiveKind, Block, Error};

#[derive(Debug)]
pub enum PageBlock<'a> {
    Title(&'a str),
    Link(&'a str, &'a str),
    Text(Vec<&'a str>),
    Page {
        identifier: &'a str,
        title: &'a str,
        text: Vec<&'a str>,
        links: Vec<(&'a str, &'a str)>,
    },
}

impl<'a> PageBlock<'a> {
    pub fn parse(block: Block<'a>) -> Result<(usize, PageBlock), Vec<(usize, Error)>> {
        let internal = match block.kind {
            BlockKind::Internal(i) => i,
            _ => todo!(),
        };

        match internal.kind {
            DirectiveKind::Page => Self::page(block.line, internal.argument, internal.children),
            DirectiveKind::Title => Self::title(block.line, internal.argument, internal.children),
            DirectiveKind::Link => Self::link(block.line, internal.argument, internal.children),
            DirectiveKind::Text => Self::text(block.line, internal.argument, internal.children),
        }
    }

    fn page(
        line: usize,
        argument: Option<&'a str>,
        children: Vec<Block<'a>>,
    ) -> Result<(usize, PageBlock<'a>), Vec<(usize, Error)>> {
        let mut errors = Vec::new();

        let identifier = match argument {
            Some(s) => s,
            None => {
                errors.push((
                    line,
                    Error::MissingArgument {
                        block: DirectiveKind::Page,
                    },
                ));
                "{unnamed}"
            }
        };

        let mut titles = Vec::with_capacity(1);
        let mut text = Vec::new();
        let mut links = Vec::new();

        for child in children {
            match Self::parse(child) {
                Ok((_, PageBlock::Title(title))) => titles.push(title),
                Ok((_, PageBlock::Text(new_text))) => text.extend(new_text),
                Ok((_, PageBlock::Link(target, text))) => links.push((target, text)),
                Err(new_errors) => errors.extend(new_errors),
                _ => todo!(),
            }
        }

        let title = match titles.as_slice() {
            [] => {
                errors.push((
                    line,
                    Error::PageMissingTitle {
                        page: identifier.to_owned(),
                    },
                ));
                "{untitled}"
            }
            [t] => t,
            [first, ..] => {
                errors.push((
                    line,
                    Error::ExcessivePageTitles {
                        page: identifier.to_owned(),
                    },
                ));
                first
            }
        };

        if errors.is_empty() {
            Ok((
                line,
                PageBlock::Page {
                    identifier,
                    title,
                    text,
                    links,
                },
            ))
        } else {
            Err(errors)
        }
    }

    fn text(
        line: usize,
        argument: Option<&'a str>,
        children: Vec<Block<'a>>,
    ) -> Result<(usize, PageBlock<'a>), Vec<(usize, Error)>> {
        let mut errors = Vec::new();

        if argument.is_some() {
            errors.push((
                line,
                Error::UnexpectedArgument {
                    block: DirectiveKind::Text,
                },
            ))
        }

        if children.is_empty() {
            errors.push((
                line,
                Error::MissingText {
                    block: DirectiveKind::Text,
                },
            ));
        }

        let mut paragraphs = Vec::with_capacity(children.len());

        for child in children {
            match child.kind {
                BlockKind::Internal(_) => errors.push((
                    child.line,
                    Error::UnexpectedChildDirective {
                        block: DirectiveKind::Text,
                    },
                )),
                BlockKind::External(text) => paragraphs.push(text),
            }
        }

        if errors.is_empty() {
            Ok((line, PageBlock::Text(paragraphs)))
        } else {
            Err(errors)
        }
    }

    fn link(
        line: usize,
        argument: Option<&'a str>,
        children: Vec<Block<'a>>,
    ) -> Result<(usize, PageBlock<'a>), Vec<(usize, Error)>> {
        let mut errors = Vec::new();

        if argument.is_none() {
            errors.push((
                line,
                Error::MissingArgument {
                    block: DirectiveKind::Link,
                },
            ));
        }

        let child = match children.as_slice() {
            [] => {
                errors.push((
                    line,
                    Error::MissingText {
                        block: DirectiveKind::Link,
                    },
                ));
                return Err(errors);
            }
            [child] => child,
            [.., last] => {
                errors.push((
                    last.line,
                    Error::ExcessiveChildCount {
                        block: DirectiveKind::Link,
                    },
                ));
                return Err(errors);
            }
        };

        if let BlockKind::External(text) = child.kind {
            Ok((line, PageBlock::Link(argument.unwrap(), text)))
        } else {
            errors.push((
                child.line,
                Error::UnexpectedChildDirective {
                    block: DirectiveKind::Link,
                },
            ));
            Err(errors)
        }
    }

    fn title(
        line: usize,
        argument: Option<&str>,
        children: Vec<Block<'a>>,
    ) -> Result<(usize, PageBlock<'a>), Vec<(usize, Error)>> {
        let mut errors = Vec::new();

        if argument.is_some() {
            errors.push((
                line,
                Error::UnexpectedArgument {
                    block: DirectiveKind::Title,
                },
            ));
        }

        let child = match children.as_slice() {
            [] => {
                errors.push((
                    line,
                    Error::MissingText {
                        block: DirectiveKind::Title,
                    },
                ));
                return Err(errors);
            }
            [child] => child,
            [.., last] => {
                errors.push((
                    last.line,
                    Error::ExcessiveChildCount {
                        block: DirectiveKind::Title,
                    },
                ));
                return Err(errors);
            }
        };

        if let BlockKind::External(text) = child.kind {
            Ok((line, PageBlock::Title(text)))
        } else {
            errors.push((
                child.line,
                Error::UnexpectedChildDirective {
                    block: DirectiveKind::Title,
                },
            ));
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, DirectiveKind, Error, PageBlock};

    #[test]
    fn title_block_cannot_be_empty() {
        let input = Block::internal(2, DirectiveKind::Title, None, Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (2, Error::MissingText { block }) if *block == DirectiveKind::Title)
        );
    }

    #[test]
    fn title_block_cannot_have_argument() {
        let input = Block::internal(2, DirectiveKind::Title, Some("oh no!"), Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(
            matches!(&output[0], (2, Error::UnexpectedArgument { block }) if *block == DirectiveKind::Title)
        );
        assert!(
            matches!(&output[1], (2, Error::MissingText { block }) if *block == DirectiveKind::Title)
        );
    }

    #[test]
    fn title_block_cannot_have_multiple_children() {
        let input = Block::internal(
            2,
            DirectiveKind::Title,
            None,
            vec![Block::external(3, "oh no"), Block::external(4, "oh no 2")],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (4, Error::ExcessiveChildCount { block }) if *block == DirectiveKind::Title )
        );
    }

    #[test]
    fn title_block_cannot_have_non_text_children() {
        let input = Block::internal(
            2,
            DirectiveKind::Title,
            None,
            vec![Block::internal(3, DirectiveKind::Title, None, Vec::new())],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(
            &output[0],
            (3, Error::UnexpectedChildDirective { block }) if *block == DirectiveKind::Title
        ));
    }

    #[test]
    fn can_parse_valid_title_block() {
        let input = Block::internal(
            2,
            DirectiveKind::Title,
            None,
            vec![Block::external(3, "hurrah!")],
        );

        let output = PageBlock::parse(input).unwrap();

        assert!(matches!(output, (2, PageBlock::Title("hurrah!"))));
    }

    #[test]
    fn link_block_requires_argument_and_children() {
        let input = Block::internal(2, DirectiveKind::Link, None, Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(matches!(
            &output[0],
            (
                2,
                Error::MissingArgument {
                    block: DirectiveKind::Link
                }
            )
        ));
        assert!(
            matches!(&output[1], (2, Error::MissingText { block }) if *block == DirectiveKind::Link)
        );
    }

    #[test]
    fn link_block_requires_children() {
        let input = Block::internal(2, DirectiveKind::Link, Some("go-to-this-page"), Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (2, Error::MissingText { block }) if *block == DirectiveKind::Link)
        );
    }

    #[test]
    fn link_block_requires_single_child() {
        let input = Block::internal(
            2,
            DirectiveKind::Link,
            Some("go-to-this-page"),
            vec![Block::external(3, "oh no"), Block::external(5, "oh no 2")],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (5, Error::ExcessiveChildCount { block }) if *block == DirectiveKind::Link )
        );
    }

    #[test]
    fn link_block_requires_text_child() {
        let input = Block::internal(
            10,
            DirectiveKind::Link,
            Some("go-to-this-page"),
            vec![Block::internal(12, DirectiveKind::Title, None, Vec::new())],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (12, Error::UnexpectedChildDirective { block }) if *block == DirectiveKind::Link )
        );
    }

    #[test]
    fn can_parse_valid_link_block() {
        let input = Block::internal(
            10,
            DirectiveKind::Link,
            Some("trip-onto-landmine"),
            vec![Block::external(12, "Watch out for that landmine!")],
        );

        let output = PageBlock::parse(input).unwrap();

        assert!(matches!(
            output,
            (10, PageBlock::Link(target, text))
                if target == "trip-onto-landmine"
                && text == "Watch out for that landmine!"
        ))
    }

    #[test]
    fn report_empty_text_blocks() {
        let input = Block::internal(123, DirectiveKind::Text, None, Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (123, Error::MissingText { block }) if *block == DirectiveKind::Text)
        );
    }

    #[test]
    fn report_unexpected_text_argument() {
        let input = Block::internal(
            4567,
            DirectiveKind::Text,
            Some("asdfghjkl"),
            vec![Block::external(2000000000, "hello")],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(
            matches!(&output[0], (4567, Error::UnexpectedArgument { block }) if *block == DirectiveKind::Text)
        );
    }

    #[test]
    fn can_parse_valid_text_blocks() {
        let input = Block::internal(
            6,
            DirectiveKind::Text,
            None,
            vec![
                Block::external(7, "first paragraph"),
                Block::external(8, "the second"),
                Block::external(9, "a third"),
                Block::external(10, "finally the fourth"),
            ],
        );

        let output = PageBlock::parse(input).unwrap();

        match output {
            (l, _) if l != 6 => panic!("Wrong line number!"),
            (_, PageBlock::Text(paragraphs)) => {
                assert_eq!(4, paragraphs.len());
                assert_eq!("first paragraph", paragraphs[0]);
                assert_eq!("the second", paragraphs[1]);
                assert_eq!("a third", paragraphs[2]);
                assert_eq!("finally the fourth", paragraphs[3]);
            }
            _ => {
                panic!("Wrong PageBlock variant!");
            }
        }
    }

    #[test]
    fn report_directives_within_text_block() {
        let input = Block::internal(
            6,
            DirectiveKind::Text,
            None,
            vec![
                Block::external(7, "first paragraph"),
                Block::external(8, "the second"),
                Block::internal(9, DirectiveKind::Text, None, Vec::new()),
                Block::external(10, "finally the fourth"),
                Block::internal(11, DirectiveKind::Text, None, Vec::new()),
            ],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(matches!(
            &output[0],
            (9, Error::UnexpectedChildDirective { block }) if *block == DirectiveKind::Text
        ));
        assert!(matches!(
            &output[1],
            (11, Error::UnexpectedChildDirective { block }) if *block == DirectiveKind::Text
        ));
    }

    #[test]
    fn report_page_without_identifier() {
        let input = Block::internal(0, DirectiveKind::Page, None, Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(matches!(
            &output[0],
            (0, Error::MissingArgument { block }) if *block == DirectiveKind::Page
        ));
        assert!(matches!(
            &output[1],
            (0, Error::PageMissingTitle { page }) if page == "{unnamed}"
        ));
    }

    #[test]
    fn report_page_without_title() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            Some("my-first-valid-page"),
            Vec::new(),
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(
            &output[0],
            (0, Error::PageMissingTitle { page }) if page == "my-first-valid-page"
        ));
    }

    #[test]
    fn report_page_with_excessive_titles() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            Some("too-many-titles"),
            vec![
                Block::internal(
                    1,
                    DirectiveKind::Title,
                    None,
                    vec![Block::external(2, "first")],
                ),
                Block::internal(
                    3,
                    DirectiveKind::Title,
                    None,
                    vec![Block::external(4, "second")],
                ),
            ],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(
            &output[0],
            (0, Error::ExcessivePageTitles { page }) if page == "too-many-titles"
        ));
    }

    #[test]
    fn can_parse_empty_valid_page() {
        let input = Block::internal(
            50,
            DirectiveKind::Page,
            Some("almost-empty"),
            vec![Block::internal(
                51,
                DirectiveKind::Title,
                None,
                vec![Block::external(52, "I have a title!")],
            )],
        );

        let output = PageBlock::parse(input).unwrap();

        match output {
            (line, _) if line != 50 => panic!("Line number is wrong"),
            (
                _,
                PageBlock::Page {
                    identifier,
                    title,
                    text,
                    links,
                },
            ) => {
                assert_eq!("almost-empty", identifier);
                assert_eq!("I have a title!", title);
                assert!(text.is_empty());
                assert!(links.is_empty());
            }
            _ => panic!("Incorrect PageBlock variant!"),
        }
    }

    #[test]
    fn can_collect_text_from_page() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            Some("with-text"),
            vec![
                Block::internal(
                    1,
                    DirectiveKind::Title,
                    None,
                    vec![Block::external(2, "Title")],
                ),
                Block::internal(
                    4,
                    DirectiveKind::Text,
                    None,
                    vec![Block::external(5, "first"), Block::external(6, "second")],
                ),
                Block::internal(
                    8,
                    DirectiveKind::Text,
                    None,
                    vec![
                        Block::external(9, "third"),
                        Block::external(10, "fourth"),
                        Block::external(11, "fifth"),
                    ],
                ),
            ],
        );

        let output = PageBlock::parse(input).unwrap();

        match output {
            (line, _) if line != 0 => panic!("Line number is wrong"),
            (
                _,
                PageBlock::Page {
                    identifier,
                    title,
                    text,
                    links,
                },
            ) => {
                assert_eq!("with-text", identifier);
                assert_eq!("Title", title);
                assert_eq!(5, text.len());
                assert_eq!("first", text[0]);
                assert_eq!("second", text[1]);
                assert_eq!("third", text[2]);
                assert_eq!("fourth", text[3]);
                assert_eq!("fifth", text[4]);
                assert!(links.is_empty());
            }
            _ => panic!("Incorrect PageBlock variant!"),
        }
    }

    #[test]
    fn inner_errors_are_reported() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            Some("with-text"),
            vec![Block::internal(1, DirectiveKind::Title, None, Vec::new())],
        );

        let output = PageBlock::parse(input).unwrap_err();

        assert!(
            matches!(&output[0], (1, Error::MissingText { block }) if *block == DirectiveKind::Title)
        );
    }

    #[test]
    fn can_collect_links_from_page() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            Some("with-links"),
            vec![
                Block::internal(
                    1,
                    DirectiveKind::Title,
                    None,
                    vec![Block::external(2, "Title 2")],
                ),
                Block::internal(
                    4,
                    DirectiveKind::Link,
                    Some("page-three"),
                    vec![Block::external(5, "Go to page three")],
                ),
                Block::internal(
                    8,
                    DirectiveKind::Link,
                    Some("page-seven"),
                    vec![Block::external(7, "Go to page seven")],
                ),
            ],
        );

        let output = PageBlock::parse(input).unwrap();

        match output {
            (line, _) if line != 0 => panic!("Line number is wrong"),
            (
                _,
                PageBlock::Page {
                    identifier,
                    title,
                    text,
                    links,
                },
            ) => {
                assert_eq!("with-links", identifier);
                assert_eq!("Title 2", title);
                assert_eq!(2, links.len());
                assert_eq!(("page-three", "Go to page three"), links[0]);
                assert_eq!(("page-seven", "Go to page seven"), links[1]);
                assert!(text.is_empty());
            }
            _ => panic!("Incorrect PageBlock variant!"),
        }
    }
}
