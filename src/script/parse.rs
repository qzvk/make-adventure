use super::Script;
use std::{
    iter::{Enumerate, Peekable},
    str::Lines,
};

#[derive(Debug, PartialEq)]
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
        for (index, line) in self.lines.by_ref() {
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

#[derive(Debug)]
struct Page<'a> {
    name: &'a str,
}

struct Pages<'a, I>
where
    I: Iterator<Item = (usize, Command<'a>)>,
{
    commands: Peekable<I>,
}

impl<'a, I> Pages<'a, I>
where
    I: Iterator<Item = (usize, Command<'a>)>,
{
    pub fn new(commands: I) -> Self {
        Self {
            commands: commands.peekable(),
        }
    }
}

impl<'a, I> Iterator for Pages<'a, I>
where
    I: Iterator<Item = (usize, Command<'a>)>,
{
    type Item = Result<Page<'a>, (usize, Error)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, command) = self.commands.next()?;

        // A top-level command has to have zero indentation.
        if command.indent_level != 0 {
            return Some(Err((
                line,
                Error::UnexpectedIndenation {
                    expected: 0,
                    found: command.indent_level,
                },
            )));
        }

        // A top-level command has to be a directive.
        let (kind, name) = match command.command {
            PlainCommand::Directive { kind, argument } => (kind, argument),
            PlainCommand::Text { .. } => return Some(Err((line, Error::TopLevelTextCommand))),
        };

        // A top-level directive has to be a page directive.
        if kind != DirectiveKind::Page {
            return Some(Err((line, Error::TopLevelNonPageDirective)));
        }

        // A page cannot have an empty name.
        if name.is_empty() {
            return Some(Err((line, Error::EmptyPageName)));
        }

        // TODO: Parse subsequent lines until we see something with indentation == 0.

        Some(Ok(Page { name }))
    }
}

pub fn parse(input: &str) -> Result<Script, Error> {
    let lines = input.lines();
    let commands_and_errors = Commands::new(lines);

    let mut errors = Vec::new();

    let commands = commands_and_errors.filter_map(|(line, next)| match next {
        Ok(c) => Some((line, c)),
        Err(e) => {
            errors.push((line, e));
            None
        }
    });

    let pages = Pages::new(commands);

    for page in pages {
        println!("{page:?}");
    }

    println!("{errors:?}");

    todo!()
}

#[derive(Debug)]
pub enum Error {
    UnexpectedIndenation { expected: usize, found: usize },
    TopLevelTextCommand,
    TopLevelNonPageDirective,
    EmptyPageName,
    InvalidIndentation { count: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
