mod babelio;
mod common;
mod google_books;

fn main() {
    let isbn = "9782757862582";
    let md = babelio::get_book_metadata_from_isbn(isbn);
    // let md = google_books::get_book_metadata_from_isbn(isbn);
    println!("md {:?}", md)
}
