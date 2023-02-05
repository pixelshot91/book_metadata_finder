use crate::{
    cached_client::CachedClient,
    common,
};
mod parser;
mod request;

pub fn get_book_metadata_from_isbn(isbn: &str) -> common::BookMetaData {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let cached_client = CachedClient {
        http_client: client,
    };
    let book_url = request::get_book_url(&cached_client, isbn);
    let book_page = request::get_book_page(&cached_client, book_url);
    let id_obj = parser::extract_id_obj(&book_page);
    let blurb = request::get_book_blurb_see_more(&cached_client, &id_obj);

    let mut res = parser::extract_title_author_keywords(&book_page);
    res.blurb = Some(blurb);
    res
}
