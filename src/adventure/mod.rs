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
    pub index: usize,
    pub paragraphs: &'a Vec<&'a str>,
    pub links: Vec<PageLink<'a>>,
}

/// Intermediate representation of an adventure, passable to handlebars for rendering.
#[derive(Debug, Serialize)]
pub struct Adventure<'a> {
    pub pages: Vec<Page<'a>>,
}

impl<'a> Adventure<'a> {
    pub fn new(script: &'a Script) -> Result<Self, Vec<Error>> {
        let mut pages = Vec::with_capacity(script.pages.len());
        let mut errors = Vec::new();

        for (index, page) in script.pages.iter().enumerate() {
            match Self::make_page(script, index + 1, page) {
                Ok(o) => pages.push(o),
                Err(e) => errors.extend(e),
            }
        }

        if errors.is_empty() {
            Ok(Self { pages })
        } else {
            Err(errors)
        }
    }

    fn make_page(
        script: &'a Script,
        index: usize,
        page: &'a script::Page,
    ) -> Result<Page<'a>, Vec<Error>> {
        let links = Self::make_links(script, page)?;

        Ok(Page {
            title: page.title,
            index,
            paragraphs: &page.paragraphs,
            links,
        })
    }

    fn make_links(
        script: &'a Script,
        info: &'a script::Page,
    ) -> Result<Vec<PageLink<'a>>, Vec<Error>> {
        let mut links = Vec::with_capacity(info.links.len());
        let mut errors = Vec::new();

        for (destination, text) in &info.links {
            match Self::find_page_index(script, destination) {
                Some(index) => links.push(PageLink { index, text }),
                None => errors.push(Error::bad_reference(info.identifier, destination)),
            }
        }

        if errors.is_empty() {
            Ok(links)
        } else {
            Err(errors)
        }
    }

    fn find_page_index(script: &'a Script, expected: &'a str) -> Option<usize> {
        for (index, page) in script.pages.iter().enumerate() {
            if page.identifier == expected {
                // Indices are offset by one, since they are meant to be read by humans.
                return Some(index + 1);
            }
        }

        None
    }
}
