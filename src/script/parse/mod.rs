mod block;
mod error;
mod line;
mod page_block;

use self::{
    block::Block,
    line::{Line, Lines},
    page_block::PageBlock,
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

fn blocks_to_page_blocks(blocks: Vec<Block>) -> Result<Vec<PageBlock>> {
    let mut page_blocks = Vec::with_capacity(blocks.len());
    let mut errors = Vec::new();

    for block in blocks {
        match PageBlock::parse(block) {
            Ok((_, o)) => page_blocks.push(o),
            Err(e) => errors.extend(e),
        }
    }

    if errors.is_empty() {
        println!("{page_blocks:#?}");
        Ok(page_blocks)
    } else {
        println!("{errors:#?}");
        Err(errors)
    }
}

fn page_blocks_to_pages(_page_blocks: Vec<PageBlock>) -> Result<Vec<Page>> {
    todo!()
}

pub fn parse(input: &str) -> Result<Script> {
    let lines = string_to_lines(input)?;
    let blocks = lines_to_blocks(lines)?;
    let page_blocks = blocks_to_page_blocks(blocks)?;
    let pages = page_blocks_to_pages(page_blocks)?;
    Ok(Script { pages })
}
