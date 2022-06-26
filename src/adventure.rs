use crate::config::Config;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PageLink<'a> {
    pub index: usize,
    pub text: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Page<'a> {
    pub title: &'a str,
    pub paragraphs: &'a Vec<String>,
    pub links: Vec<PageLink<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Adventure<'a> {
    pub pages: Vec<Page<'a>>,
}

#[derive(Debug)]
pub enum Error {
    /// Failed to lookup index of a page's link.
    BadIndex { page: String, link: String },
}

impl<'a> std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadIndex { page, link } => write!(
                f,
                "The page {page:?} links to {link:?}, which does not exist!"
            ),
        }
    }
}

impl<'a> std::error::Error for Error {}

impl<'a> Adventure<'a> {
    pub fn new(config: &'a Config) -> Result<Self, Error> {
        let mut pages = Vec::new();

        for (page_id, info) in &config.pages {
            let mut links = Vec::with_capacity(info.links.len());

            for (text, destination_id) in &info.links {
                let mut page_index = None;

                for (index, (id, _)) in config.pages.iter().enumerate() {
                    if id == destination_id {
                        // Indices are offset by one, since they are meant to be read by humans.
                        page_index = Some(index + 1);
                        break;
                    }
                }

                if let Some(index) = page_index {
                    links.push(PageLink { index, text });
                } else {
                    return Err(Error::BadIndex {
                        page: page_id.clone(),
                        link: destination_id.clone(),
                    });
                }
            }

            pages.push(Page {
                title: info.title.as_str(),
                paragraphs: &info.paragraphs,
                links,
            });
        }

        Ok(Self { pages })
    }
}
