pub mod error;

use crate::script::{self, Script};
use error::Error;
use serde::Serialize;

/// A link with a title to an indexed page.
#[derive(Debug, Serialize)]
pub struct PageLink<'a> {
    pub index: usize,
    pub text: &'a str,
}

/// A page within an adventure.
#[derive(Debug, Serialize)]
pub struct Page<'a> {
    pub title: &'a str,
    pub paragraphs: &'a Vec<String>,
    pub links: Vec<PageLink<'a>>,
}

/// Intermediate representation of an adventure, passable to handlebars for rendering.
#[derive(Debug, Serialize)]
pub struct Adventure<'a> {
    pub pages: Vec<Page<'a>>,
}

impl<'a> Adventure<'a> {
    pub fn new(script: &'a Script) -> Result<Self, Error> {
        let mut pages = Vec::with_capacity(script.pages.len());

        for page in &script.pages {
            let page = Self::make_page(script, page)?;
            pages.push(page);
        }

        Ok(Self { pages })
    }

    fn make_page(config: &'a Script, page: &'a script::Page) -> Result<Page<'a>, Error> {
        let links = Self::make_links(page)?;

        Ok(Page {
            title: page.title.as_str(),
            paragraphs: &page.paragraphs,
            links,
        })
    }

    fn make_links(page: &'a script::Page) -> Result<Vec<PageLink<'a>>, Error> {
        let mut links = Vec::with_capacity(page.links.len());

        for (text, &index) in &page.links {
            links.push(PageLink { index, text });
        }

        Ok(links)
    }
}
