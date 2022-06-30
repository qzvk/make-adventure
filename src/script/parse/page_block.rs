use super::{block::BlockKind, line::DirectiveKind, Block, Error};

#[derive(Debug)]
enum PageBlock<'a> {
    Title(&'a str),
}

impl<'a> PageBlock<'a> {
    pub fn parse(block: Block) -> Result<(usize, PageBlock), (usize, Error)> {
        let internal = match block.kind {
            BlockKind::Internal(i) => i,
            _ => todo!(),
        };

        if internal.argument.is_some() {
            return Err((block.line, Error::UnexpectedTitleArgument));
        }

        let child = match internal.children.as_slice() {
            [] => return Err((block.line, Error::MissingTitleText)),
            [child] => child,
            [.., last] => return Err((last.line, Error::ExcessiveTitleText)),
        };

        if let BlockKind::External(text) = child.kind {
            Ok((block.line, PageBlock::Title(text)))
        } else {
            Err((child.line, Error::UnexpectedChildDirectiveOfTitle))
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

        assert!(matches!(output, (2, Error::MissingTitleText)));
    }

    #[test]
    fn title_block_cannot_have_argument() {
        let input = Block::internal(2, DirectiveKind::Title, Some("oh no!"), Vec::new());

        let output = PageBlock::parse(input).unwrap_err();

        assert!(matches!(output, (2, Error::UnexpectedTitleArgument)));
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

        assert!(matches!(output, (4, Error::ExcessiveTitleText)));
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

        assert!(matches!(
            output,
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
