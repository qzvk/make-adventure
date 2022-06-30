use super::{
    block::{BlockKind, InternalBlock},
    line::DirectiveKind,
    Block, Error,
};

#[derive(Debug)]
enum PageBlock<'a> {
    Title(&'a str),
    Link(&'a str, &'a str),
}

impl<'a> PageBlock<'a> {
    pub fn parse(block: Block<'a>) -> Result<(usize, PageBlock), Vec<(usize, Error)>> {
        let internal = match block.kind {
            BlockKind::Internal(i) => i,
            _ => todo!(),
        };

        match internal.kind {
            DirectiveKind::Page => todo!(),
            DirectiveKind::Title => Self::title(block.line, internal.argument, internal.children),
            DirectiveKind::Link => Self::link(block.line, internal.argument, internal.children),
            DirectiveKind::Text => todo!(),
        }
    }

    fn link(
        line: usize,
        argument: Option<&'a str>,
        children: Vec<Block<'a>>,
    ) -> Result<(usize, PageBlock<'a>), Vec<(usize, Error)>> {
        let mut errors = Vec::new();

        if argument.is_none() {
            errors.push((line, Error::MissingLinkArgument));
        }

        let child = match children.as_slice() {
            [] => {
                errors.push((line, Error::MissingLinkText));
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
            errors.push((line, Error::UnexpectedTitleArgument));
        }

        let child = match children.as_slice() {
            [] => {
                errors.push((line, Error::MissingTitleText));
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
        assert!(matches!(&output[0], (2, Error::MissingTitleText)));
    }

    #[test]
    fn title_block_cannot_have_argument() {
        let input = Block::internal(2, DirectiveKind::Title, Some("oh no!"), Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(matches!(&output[0], (2, Error::UnexpectedTitleArgument)));
        assert!(matches!(&output[1], (2, Error::MissingTitleText)));
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
        assert!(matches!(&output[0], (2, Error::MissingLinkArgument)));
        assert!(matches!(&output[1], (2, Error::MissingLinkText)));
    }

    #[test]
    fn link_block_requires_children() {
        let input = Block::internal(2, DirectiveKind::Link, Some("go-to-this-page"), Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(&output[0], (2, Error::MissingLinkText)));
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
}
