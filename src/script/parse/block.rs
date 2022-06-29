use std::iter::Peekable;

use super::{
    line::{DirectiveKind, Line, LineKind},
    Error,
};

#[derive(Debug, PartialEq)]
pub struct InternalBlock<'a> {
    pub kind: DirectiveKind,
    pub argument: Option<&'a str>,
    pub children: Vec<Block<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum BlockKind<'a> {
    Internal(InternalBlock<'a>),
    External(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub line: usize,
    pub kind: BlockKind<'a>,
}

impl<'a> Block<'a> {
    pub fn parse(lines: Vec<(usize, Line<'a>)>) -> Result<Vec<Block<'a>>, Vec<(usize, Error)>> {
        let mut lines = lines.into_iter().peekable();
        let mut errors = Vec::new();
        let blocks = Self::parse_indented(0, &mut lines, &mut errors);

        if errors.is_empty() {
            Ok(blocks)
        } else {
            Err(errors)
        }
    }

    pub fn internal(
        line: usize,
        kind: DirectiveKind,
        argument: Option<&'a str>,
        children: Vec<Block<'a>>,
    ) -> Self {
        Self {
            line,
            kind: BlockKind::Internal(InternalBlock {
                kind,
                argument,
                children,
            }),
        }
    }

    pub fn external(line: usize, text: &'a str) -> Self {
        Self {
            line,
            kind: BlockKind::External(text),
        }
    }

    fn parse_indented<I>(
        indent: usize,
        lines: &mut Peekable<I>,
        errors: &mut Vec<(usize, Error)>,
    ) -> Vec<Block<'a>>
    where
        I: Iterator<Item = (usize, Line<'a>)>,
    {
        let mut blocks = Vec::new();

        // Consume commands that are at least the current indent level.
        while let Some((number, line)) = lines.next_if(|(_, line)| line.indent >= indent) {
            // If something is _too_ indented, report an error.
            if line.indent > indent {
                // If the indentation is wrong all bets are off. So not to spam the user with
                // errors, consume all the rest of the indented lines and return now.
                errors.push((
                    number,
                    Error::UnexpectedIndenation {
                        expected: indent,
                        found: line.indent,
                    },
                ));

                while lines.next_if(|(_, line)| line.indent >= indent).is_some() {}
                return Vec::new();
            }

            match line.kind {
                LineKind::Text(text) => {
                    blocks.push(Block::external(number, text));
                }
                LineKind::Directive(kind, argument) => {
                    let children = Self::parse_indented(indent + 1, lines, errors);
                    let new_block = Block::internal(number, kind, argument, children);
                    blocks.push(new_block);
                }
            }
        }

        blocks
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{line::DirectiveKind, string_to_lines, Error},
        Block, Line,
    };

    #[test]
    fn can_parse_text_lines() {
        const INPUT: &[(usize, usize, &str)] = &[
            (1, 0, "Hello, world!"),
            (3, 0, "Another string"),
            (5, 0, ""),
            (100, 0, "Hello, world!"),
        ];

        let input: Vec<_> = INPUT
            .iter()
            .map(|&(number, indent, text)| (number, Line::new_text(indent, text)))
            .collect();

        let actual = Block::parse(input).unwrap();

        let expected: Vec<Block> = INPUT
            .iter()
            .map(|&(line, _, text)| Block::external(line, text))
            .collect();

        assert_eq!(expected.len(), actual.len());
        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn can_parse_complex_block() {
        let input = "page dungeon-entrance
    title
        The Dungeon Entrance

    text
        The dungeon! Eek!

        I've decided - I'm having two paragraphs... Pass the detritus!

    link dungeon-locked-door
        Try the door.

    link stumble-off-cliff
        Flee.
";

        let expected = vec![Block::internal(
            0,
            DirectiveKind::Page,
            Some("dungeon-entrance"),
            vec![
                Block::internal(
                    1,
                    DirectiveKind::Title,
                    None,
                    vec![Block::external(2, "The Dungeon Entrance")],
                ),
                Block::internal(
                    4,
                    DirectiveKind::Text,
                    None,
                    vec![
                        Block::external(5, "The dungeon! Eek!"),
                        Block::external(
                            7,
                            "I've decided - I'm having two paragraphs... Pass the detritus!",
                        ),
                    ],
                ),
                Block::internal(
                    9,
                    DirectiveKind::Link,
                    Some("dungeon-locked-door"),
                    vec![Block::external(10, "Try the door.")],
                ),
                Block::internal(
                    12,
                    DirectiveKind::Link,
                    Some("stumble-off-cliff"),
                    vec![Block::external(13, "Flee.")],
                ),
            ],
        )];

        let lines = string_to_lines(input).unwrap();
        let actual = Block::parse(lines).unwrap();

        assert_eq!(expected.len(), actual.len());

        for (expected, actual) in expected.iter().zip(actual.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn can_report_bad_indentation() {
        let input = "page first-one
        title
            I just indented twice in one go! Bad!
            These lines should get skipped, since they're in a badly indented block!

page second-try
    link this-ones-okay
                But not this!
";

        let expected = vec![(1, 1, 2), (7, 2, 4)];

        let lines = string_to_lines(input).unwrap();
        let actual = Block::parse(lines).unwrap_err();

        assert_eq!(expected.len(), actual.len());

        // Oh, wow. This isn't even a little confusing.
        // We need to check if the error (which reports an expected and actual indentations has
        // correct expectations, which is why theres an expected expected, etc, etc.

        for ((line, expected_expected, expected_found), (actual_line, actual_error)) in
            expected.iter().zip(actual.iter())
        {
            assert_eq!(line, actual_line);
            assert!(matches!(
                actual_error,
                Error::UnexpectedIndenation { expected, found } if
                    expected == expected_expected && found == expected_found
            ))
        }
    }
}
