#[derive(Default, Debug, PartialEq)]
pub struct BookMetaData {
    pub title: Option<String>,
    pub author: Option<String>,
    // A book blurb is a short promotional description.
    // A synopsis summarizes the twists, turns, and conclusion of the story.
    pub blurb: Option<String>,
    pub key_words: Option<Vec<String>>,
}
