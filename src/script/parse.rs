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

pub fn parse(input: &str) -> Result<Script, Error> {
    let lines = input.lines();
    let commands_and_errors = Commands::new(lines);

    let mut errors = Vec::new();

    let mut commands = commands_and_errors
        .filter_map(|(line, next)| match next {
            Ok(c) => Some((line, c)),
            Err(e) => {
                errors.push((line, e));
                None
            }
        })
        .peekable();

    let blocks = Block::new(&mut commands);

    if !errors.is_empty() {
        println!("Command errors");
        for (line, error) in &errors {
            println!("    [{line}] {error:?}");
        }
    }

    if let Err(block_errors) = &blocks {
        println!("Block errors");
        for (_line, error) in block_errors {
            println!("    {error:?}");
        }
    }

    if let Ok(blocks) = blocks {
        println!("Blocks");
        for block in blocks {
            println!("    {block:?}");
        }
    }

    todo!()
}

#[derive(Debug)]
struct InternalBlock<'a> {
    line: usize,
    kind: DirectiveKind,
    argument: &'a str,
    children: Vec<Block<'a>>,
}

#[derive(Debug)]
struct ExternalBlock<'a> {
    line: usize,
    text: &'a str,
}

#[derive(Debug)]
enum Block<'a> {
    Internal(InternalBlock<'a>),
    External(ExternalBlock<'a>),
}

impl<'a> Block<'a> {
    #[inline]
    pub fn new<I>(commands: &mut Peekable<I>) -> Result<Vec<Block<'a>>, Vec<(usize, Error)>>
    where
        I: Iterator<Item = (usize, Command<'a>)>,
    {
        Self::new_indented(0, commands)
    }

    fn internal(
        line: usize,
        kind: DirectiveKind,
        argument: &'a str,
        children: Vec<Block<'a>>,
    ) -> Self {
        Self::Internal(InternalBlock {
            line,
            kind,
            argument,
            children,
        })
    }

    fn external(line: usize, text: &'a str) -> Self {
        Self::External(ExternalBlock { line, text })
    }

    fn new_indented<I>(
        indent: usize,
        commands: &mut Peekable<I>,
    ) -> Result<Vec<Block<'a>>, Vec<(usize, Error)>>
    where
        I: Iterator<Item = (usize, Command<'a>)>,
    {
        let mut errors = Vec::new();
        let mut blocks = Vec::new();

        // Consume commands that are at least the current indent level.
        while let Some((line, command)) =
            commands.next_if(|(_, command)| command.indent_level >= indent)
        {
            // If something is _too_ indented, report an error.
            if command.indent_level > indent {
                errors.push((
                    line,
                    Error::UnexpectedIndenation {
                        expected: indent,
                        found: command.indent_level,
                    },
                ));
                continue;
            }

            match command.command {
                PlainCommand::Directive { kind, argument } => {
                    match Self::new_indented(indent + 1, commands) {
                        Ok(children) => {
                            blocks.push(Block::internal(line, kind, argument, children))
                        }
                        Err(new_errors) => {
                            errors.extend(new_errors);
                        }
                    }
                }
                PlainCommand::Text { raw } => {
                    blocks.push(Block::external(line, raw));
                }
            }
        }

        if errors.is_empty() {
            Ok(blocks)
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedIndenation { expected: usize, found: usize },
    InvalidIndentation { count: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
