use super::{block::BlockKind, line::DirectiveKind, Block, Error};

#[derive(Debug)]
enum PageBlock<'a> {
    Title(&'a str),
}

impl<'a> PageBlock<'a> {
    pub fn parse(block: Block) -> Result<(usize, PageBlock), Vec<(usize, Error)>> {
        let internal = match block.kind {
            BlockKind::Internal(i) => i,
            _ => todo!(),
        };

        let mut errors = Vec::new();

        if internal.argument.is_some() {
            errors.push((block.line, Error::UnexpectedTitleArgument));
        }

        let child = match internal.children.as_slice() {
            [] => {
                errors.push((block.line, Error::MissingTitleText));
                return Err(errors);
            }
            [child] => child,
            [.., last] => {
                errors.push((last.line, Error::ExcessiveTitleText));
                return Err(errors);
            }
        };

        if let BlockKind::External(text) = child.kind {
            Ok((block.line, PageBlock::Title(text)))
        } else {
            errors.push((child.line, Error::UnexpectedChildDirectiveOfTitle));
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
        assert!(matches!(&output[0], (4, Error::ExcessiveTitleText)));
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
            (3, Error::UnexpectedChildDirectiveOfTitle)
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
}
