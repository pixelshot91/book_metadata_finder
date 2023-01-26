use crate::common;
mod parser;
mod request;

fn get_book_metadata_from_url(
    client: &reqwest::blocking::Client,
    url: String,
) -> common::BookMetaData {
    let book_page = request::get_book_page(client, url);
    let id_obj = parser::extract_id_obj(book_page);
    let blurb = request::get_book_blurb_see_more(client, &id_obj);

    common::BookMetaData {
        blurb: Some(blurb),
        ..Default::default()
    }
}

pub fn get_book_metadata_from_isbn(isbn: &str) -> common::BookMetaData {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let book_url = request::get_book_url(&client, isbn);
    println!("book_url {:#?}", book_url);
    let metadata = get_book_metadata_from_url(&client, book_url);
    println!("BookMetaData {:#?}", metadata);
    metadata
}
