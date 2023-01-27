mod babelio;
mod google_books;
mod common;

fn main() {
    // babelio::get_book_metadata_from_isbn("9782266071529");
    google_books::get_book_metadata_from_isbn("9782744170812");
}
