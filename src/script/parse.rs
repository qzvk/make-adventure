use super::Script;
use std::{iter::Enumerate, str::Lines};

#[derive(Debug)]
enum DirectiveKind {
    Page,
    Title,
    Text,
    Link,
}

#[derive(Debug)]
enum PlainCommand<'a> {
    Directive {
        kind: DirectiveKind,
        argument: &'a str,
    },
    Text {
        raw: &'a str,
    },
}

impl DirectiveKind {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "page" => Some(DirectiveKind::Page),
            "title" => Some(DirectiveKind::Title),
            "text" => Some(DirectiveKind::Text),
            "link" => Some(DirectiveKind::Link),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Command<'a> {
    command: PlainCommand<'a>,
    indent_level: usize,
}

impl<'a> Command<'a> {
    /// Parse a line into a command. This can succeed and return a command, succeed and return
    /// nothing (empty lines), or fail and return an error.
    pub fn parse(line: &'a str) -> Result<Option<Self>, Error> {
        let (indent_level, command) = Self::take_whitespace(line)?;

        let (word, rest) = Self::take_word(command);

        // We never hit anything but whitespace, we can ignore this line.
        if word.is_empty() {
            return Ok(None);
        }

        let plain_command = match DirectiveKind::from_str(word) {
            Some(kind) => PlainCommand::Directive {
                kind,
                argument: Self::skip_whitespace(rest),
            },
            None => PlainCommand::Text { raw: command },
        };

        Ok(Some(Self {
            command: plain_command,
            indent_level,
        }))
    }

    /// Attempt to read `4n` space characters. On success, return `n` and the rest of the string.
    /// Otherwise, return the encountered errors.
    fn take_whitespace(input: &str) -> Result<(usize, &str), Error> {
        let mut count = 0;

        for (index, c) in input.char_indices() {
            if c == ' ' {
                count += 1;
            } else if (count % 4) == 0 {
                let rest = &input[index..];
                return Ok((count / 4, rest));
            } else {
                return Err(Error::InvalidIndentation { count });
            }
        }

        // If the line only contained spaces, it'll be discared as an empty line. Reporting an
        // empty line which happens to have bad indentation will be annoying, so we silently pass
        // them along.
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

    /// Skip whitespace characters, returning the string starting at the first non-whitespace character.
    fn skip_whitespace(input: &str) -> &str {
        for (index, c) in input.char_indices() {
            if !c.is_whitespace() {
                return &input[index..];
            }
        }

        // The string is only whitespace.
        ""
    }
}

struct Commands<'a> {
    lines: Enumerate<Lines<'a>>,
}

impl<'a> Iterator for Commands<'a> {
    type Item = (usize, Result<Command<'a>, Error>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, line)) = self.lines.next() {
            let command = Command::parse(line);

            match command {
                Ok(Some(c)) => return Some((index, Ok(c))),
                Err(e) => return Some((index, Err(e))),
                _ => {}
            };
        }

        None
    }
}

impl<'a> Commands<'a> {
    pub fn new(lines: Lines<'a>) -> Self {
        Self {
            lines: lines.enumerate(),
        }
    }
}

pub fn parse(input: &str) -> Result<Script, Error> {
    let commands = Commands::new(input.lines());

    for command in commands {
        println!("{command:?}");
    }

    todo!()
}

#[derive(Debug)]
pub enum Error {
    InvalidIndentation { count: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
