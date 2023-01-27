use crate::common;
mod parser;
mod request;

pub fn get_book_metadata_from_isbn(isbn: &str) -> common::BookMetaData {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let book_url = request::get_book_url(&client, isbn);
    let book_page = request::get_book_page(&client, book_url);
    let id_obj = parser::extract_id_obj(book_page);
    let blurb = request::get_book_blurb_see_more(&client, &id_obj);

    common::BookMetaData {
        blurb: Some(blurb),
        ..Default::default()
    }
}
