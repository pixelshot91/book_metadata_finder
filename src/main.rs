mod babelio;
mod common;
mod google_books;
mod cached_client;

fn main() {
    let isbn = "9782757862582";
    let isbn = "9782266023160";
    let md = babelio::get_book_metadata_from_isbn(isbn);
    // let md = google_books::get_book_metadata_from_isbn(isbn);
    println!("md {:?}", md)
}
