use std::iter::Enumerate;

use super::Error;

#[derive(Debug, PartialEq)]
pub enum DirectiveKind {
    Page,
    Title,
    Link,
    Text,
}

impl DirectiveKind {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "page" => Some(Self::Page),
            "title" => Some(Self::Title),
            "link" => Some(Self::Link),
            "text" => Some(Self::Text),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LineKind<'a> {
    Text(&'a str),
    Directive(DirectiveKind, Option<&'a str>),
}

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub indent: usize,
    pub kind: LineKind<'a>,
}

impl<'a> Line<'a> {
    /// Parse a single line from a string. An empty line is considered successful, which
    /// returns `Ok(None)`.
    pub fn parse(line: &'a str) -> Result<Option<Self>, Error> {
        let (indent, command) = Self::take_whitespace(line)?;

        let (word, rest) = Self::take_word(command);

        // We never hit anything but whitespace, we can ignore this line.
        if word.is_empty() {
            return Ok(None);
        }

        let kind = match DirectiveKind::from_str(word) {
            Some(kind) => LineKind::Directive(kind, Self::skip_whitespace(rest)),
            None => LineKind::Text(command),
        };

        Ok(Some(Self { indent, kind }))
    }

    pub const fn new_text(indent: usize, text: &'a str) -> Self {
        Self {
            indent,
            kind: LineKind::Text(text),
        }
    }

    pub const fn new_directive(indent: usize, kind: DirectiveKind, text: Option<&'a str>) -> Self {
        Self {
            indent,
            kind: LineKind::Directive(kind, text),
        }
    }

    /// Attempt to read `4n` space characters. On success, return `n` and the rest of the string.
    /// Otherwise, return the encountered errors.
    fn take_whitespace(input: &str) -> Result<(usize, &str), Error> {
        let mut count = 0;

        for (index, c) in input.char_indices() {
            if c == ' ' {
                count += 1;
            } else if c == '#' {
                // This is a comment line, so we'll report it as empty. We can ignore potentially
                // incorrect indentation of comments, as they don't matter syntactically.
                return Ok((count / 4, ""));
            } else if (count % 4) == 0 {
                let rest = &input[index..];
                return Ok((count / 4, rest));
            } else {
                return Err(Error::InvalidIndentation { count });
            }
        }

        // If the string only contained spaces, it'll be discared as empty. Reporting an empty line
        // which happens to have bad indentation will be annoying, so we silently pass them along.
        Ok((count / 4, ""))
    }

    /// Read the string until the first whitespace character (exclusive). Return the read string,
    /// along with the rest of the string.
    fn take_word(input: &str) -> (&str, &str) {
        for (index, c) in input.char_indices() {
            if c.is_whitespace() {
                return (&input[..index], &input[index..]);
            }
        }

        // We read the whole of the string without hitting whitespace.
        (input, "")
    }

    /// Return the string before a comment, if any.
    fn trim_comment(input: &str) -> &str {
        if let Some((before, _)) = input.split_once(|c| c == '#') {
            before
        } else {
            input
        }
    }

    /// return `None` if the string is empty, otherwise return `Some(string)`.
    fn non_empty(string: &str) -> Option<&str> {
        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }

    /// Skip whitespace characters, returning the string starting at the first non-whitespace character.
    fn skip_whitespace(input: &str) -> Option<&str> {
        Self::non_empty(Self::trim_comment(input).trim())
    }
}

pub struct Lines<'a> {
    inner: Enumerate<std::str::Lines<'a>>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = (usize, Result<Line<'a>, Error>);

    fn next(&mut self) -> Option<Self::Item> {
        for (index, string) in self.inner.by_ref() {
            let parsed = Line::parse(string);

            match parsed {
                Ok(Some(l)) => return Some((index, Ok(l))),
                Err(e) => return Some((index, Err(e))),
                _ => {}
            }
        }

        None
    }
}

impl<'a> Lines<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            inner: string.lines().enumerate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Error, DirectiveKind, Line, Lines};

    #[test]
    fn can_get_directive_kinds() {
        const EXAMPLES: &[(&str, Option<DirectiveKind>)] = &[
            ("page", Some(DirectiveKind::Page)),
            ("title", Some(DirectiveKind::Title)),
            ("link", Some(DirectiveKind::Link)),
            ("text", Some(DirectiveKind::Text)),
            ("", None),
            ("pag", None),
            ("links", None),
            ("hdewuid", None),
        ];

        for (input, expected) in EXAMPLES {
            let actual = DirectiveKind::from_str(input);
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn can_parse_empty_lines() {
        const EMPTY_LINES: &[&str] = &["", " ", "  ", "   ", "    ", "     "];

        for line in EMPTY_LINES {
            let result = Line::parse(line);
            let matches = matches!(result, Ok(None));
            assert!(matches);
        }
    }

    #[test]
    fn can_parse_comments() {
        const COMMENTS: &[&str] = &["# ...", "     # nothing!"];

        for line in COMMENTS {
            let result = Line::parse(line);
            assert!(matches!(result, Ok(None)));
        }
    }

    #[test]
    fn can_parse_directives() {
        const EXAMPLES: &[(&str, Line)] = &[
            ("page", Line::new_directive(0, DirectiveKind::Page, None)),
            (
                "        title Hello!",
                Line::new_directive(2, DirectiveKind::Title, Some("Hello!")),
            ),
            (
                "link Goodbye :(",
                Line::new_directive(0, DirectiveKind::Link, Some("Goodbye :(")),
            ),
            (
                "    text    ",
                Line::new_directive(1, DirectiveKind::Text, None),
            ),
            (
                "            link whitespace  ",
                Line::new_directive(3, DirectiveKind::Link, Some("whitespace")),
            ),
            (
                "    title Another directive example   #   and comments!?   ",
                Line::new_directive(1, DirectiveKind::Title, Some("Another directive example")),
            ),
        ];

        for (input, expected) in EXAMPLES {
            let actual = Line::parse(input).unwrap().unwrap();
            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn can_generate_indentation_errors() {
        const EXAMPLES: &[(&str, usize)] = &[
            (" a", 1),
            ("  a", 2),
            ("   a", 3),
            ("     a", 5),
            ("                 a", 17),
            ("                             a", 29),
        ];

        for (input, expected) in EXAMPLES {
            let actual = Line::parse(input).unwrap_err();
            assert!(matches!(
                actual,
                Error::InvalidIndentation { count } if count == *expected
            ));
        }
    }

    #[test]
    fn empty_string_has_no_lines() {
        let input = "\n\n    \n\n               \n\n\n\n    \n";
        let output: Vec<_> = Lines::new(input).collect();
        assert!(output.is_empty());
    }
}
