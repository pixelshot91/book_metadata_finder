#[derive(Debug)]
pub struct BookMetaData {
    pub title: String,
    pub author: String,
    // A book blurb is a short promotional description.
    // A synopsis summarizes the twists, turns, and conclusion of the story.
    pub blurb: String,
    pub key_words: Vec<String>,
}
