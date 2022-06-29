use super::{
    block::{Block, BlockKind},
    line::DirectiveKind,
    Error,
};

#[derive(Debug)]
pub struct PageBlock;

impl PageBlock {
    pub fn new(block: Block) -> Result<Self, Vec<Error>> {
        let block = match block.kind {
            BlockKind::External(_) => return Err(vec![Error::TextAtTopLevel]),
            BlockKind::Internal(i) => i,
        };

        if block.kind != DirectiveKind::Page {
            return Err(vec![Error::UnexpectedTopLevelDirective {
                found: block.kind,
            }]);
        }

        // Now we know this is, at the very least, an actual page, we can start accumulating
        // errors properly.
        let mut errors = Vec::new();

        if block.argument.is_none() {
            errors.push(Error::MissingPageName);
        }

        if block.children.is_empty() {
            errors.push(Error::MissingTitleDirective {
                page: block.argument.unwrap_or("{unnamed}").to_owned(),
            });
        }

        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::line::DirectiveKind, Block, Error, PageBlock};

    #[test]
    fn reports_top_level_text() {
        let input = Block::external(15, "You're not supposed to be here.");
        let output = PageBlock::new(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(output[0], Error::TextAtTopLevel));
    }

    #[test]
    fn reports_top_level_title_directives() {
        for directive in [
            DirectiveKind::Link,
            DirectiveKind::Text,
            DirectiveKind::Title,
        ] {
            let input = Block::internal(0, DirectiveKind::Title, None, Vec::new());

            let output = PageBlock::new(input).unwrap_err();

            assert_eq!(1, output.len());
            assert!(matches!(
                output[0],
                Error::UnexpectedTopLevelDirective { found } if found == DirectiveKind::Title
            ));
        }
    }

    #[test]
    fn reports_absent_title_directive() {
        let input = Block::internal(0, DirectiveKind::Page, Some("My page!"), Vec::new());

        let output = PageBlock::new(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(
            &output[0],
            Error::MissingTitleDirective { page } if page == "My page!"
        ));
    }

    #[test]
    fn reports_absent_page_name() {
        let input = Block::internal(
            0,
            DirectiveKind::Page,
            None,
            vec![Block::internal(
                1,
                DirectiveKind::Title,
                None,
                vec![Block::external(2, "title text :(")],
            )],
        );

        let output = PageBlock::new(input).unwrap_err();

        assert_eq!(1, output.len());
        assert!(matches!(&output[0], Error::MissingPageName));
    }

    #[test]
    fn reports_missing_title_and_name() {
        let input = Block::internal(0, DirectiveKind::Page, None, Vec::new());

        let output = PageBlock::new(input).unwrap_err();

        assert_eq!(2, output.len());
        assert!(matches!(&output[0], Error::MissingPageName));
        assert!(matches!(&output[1], Error::MissingTitleDirective { .. }));
    }
}
