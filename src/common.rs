#[derive(Default, Debug, PartialEq)]
pub struct BookMetaData {
    pub title: Option<String>,
    pub authors: Option<Vec<Author>>,
    // A book blurb is a short promotional description.
    // A synopsis summarizes the twists, turns, and conclusion of the story.
    pub blurb: Option<String>,
    pub key_words: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Author {
    pub first_name: String,
    pub last_name: String,
}

pub fn html_select(sel: &str) -> scraper::Selector {
    scraper::Selector::parse(sel).unwrap()
}
