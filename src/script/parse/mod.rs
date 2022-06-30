mod block;
mod error;
mod line;
mod page;

use self::{
    block::Block,
    line::{Line, Lines},
    page::PageBlock,
};
use super::{Page, Script};
pub use error::Error;

type Result<T> = std::result::Result<T, Vec<(usize, Error)>>;

fn string_to_lines(string: &str) -> Result<Vec<(usize, Line)>> {
    let mut lines = Vec::new();
    let mut errors = Vec::new();

    for (number, result) in Lines::new(string) {
        match result {
            Ok(o) => lines.push((number, o)),
            Err(e) => errors.push((number, e)),
        }
    }

    if errors.is_empty() {
        Ok(lines)
    } else {
        Err(errors)
    }
}

fn lines_to_blocks(lines: Vec<(usize, Line)>) -> Result<Vec<Block>> {
    Block::parse(lines)
}

fn blocks_to_pages(blocks: Vec<Block>) -> Result<Vec<Page>> {
    let mut pages = Vec::with_capacity(blocks.len());
    let mut errors = Vec::new();

    for block in blocks {
        match PageBlock::parse(block) {
            Ok((_, PageBlock::Page(page))) => pages.push(page),
            Ok((n, _)) => errors.push((n, Error::NonPageTopLevelBlock)),
            Err(e) => errors.extend(e),
        }
    }

    if errors.is_empty() {
        Ok(pages)
    } else {
        Err(errors)
    }
}

pub fn parse(input: &str) -> Result<Script> {
    let lines = string_to_lines(input)?;
    let blocks = lines_to_blocks(lines)?;
    let pages = blocks_to_pages(blocks)?;
    Ok(Script { pages })
}
