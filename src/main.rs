mod babelio;
mod cached_client;
mod common;
mod google_books;

fn main() {
    let isbn = "9782757862582";
    let isbn = "9782266023160";
    let isbn = "9782253029854";
    let md = babelio::get_book_metadata_from_isbn(isbn);
    // let md = google_books::get_book_metadata_from_isbn(isbn);
    println!("md {:?}", md)
}
