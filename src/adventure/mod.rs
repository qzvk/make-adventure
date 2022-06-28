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

        for (id, info) in &script.pages {
            let page = Self::make_page(script, id, info)?;
            pages.push(page);
        }

        Ok(Self { pages })
    }

    fn make_page(
        config: &'a Script,
        id: &'a str,
        info: &'a script::Page,
    ) -> Result<Page<'a>, Error> {
        let links = Self::make_links(config, id, info)?;

        Ok(Page {
            title: info.title.as_str(),
            paragraphs: &info.paragraphs,
            links,
        })
    }

    fn make_links(
        config: &'a Script,
        id: &'a str,
        info: &'a script::Page,
    ) -> Result<Vec<PageLink<'a>>, Error> {
        let mut links = Vec::with_capacity(info.links.len());

        for (text, destination_id) in &info.links {
            let index = Self::find_page_index(config, destination_id)
                .ok_or_else(|| Error::bad_index(id, destination_id))?;

            links.push(PageLink { index, text });
        }

        Ok(links)
    }

    fn find_page_index(config: &'a Script, expected_id: &'a str) -> Option<usize> {
        for (index, (id, _)) in config.pages.iter().enumerate() {
            if id == expected_id {
                // Indices are offset by one, since they are meant to be read by humans.
                return Some(index + 1);
            }
        }

        None
    }
}
