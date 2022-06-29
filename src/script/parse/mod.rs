mod block;
mod error;
mod line;

use self::{
    block::Block,
    line::{Line, Lines},
};
use super::{Page, Script};
pub use error::Error;

struct PageBlock {}

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
    let result = Block::parse(lines)?;
    println!("{result:?}");
    todo!()
}

fn blocks_to_page_blocks(blocks: Vec<Block>) -> Result<Vec<PageBlock>> {
    todo!()
}

fn page_blocks_to_pages(page_blocks: Vec<PageBlock>) -> Result<Vec<Page>> {
    todo!()
}

pub fn parse(input: &str) -> Result<Script> {
    let lines = string_to_lines(input)?;
    let blocks = lines_to_blocks(lines)?;
    let page_blocks = blocks_to_page_blocks(blocks)?;
    let pages = page_blocks_to_pages(page_blocks)?;
    Ok(Script { pages })
}
