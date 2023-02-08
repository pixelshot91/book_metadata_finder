mod babelio;
mod cached_client;
mod common;
mod google_books;
use std::env;
use std::process::Command;
use std::str;

use itertools::Itertools;

fn main() {
    let isbns: Vec<String> = env::args()
        .skip(1)
        .map(|picture_path| {
            println!("{picture_path}");
            let output = Command::new(
            "/home/julien/Perso/LeBonCoin/chain_automatisation/book_metadata_finder/detect_barcode",
                )
                .arg("-in=".to_string() + &picture_path)
                .output()
                .expect("failed to execute process");
            let output = str::from_utf8(&output.stdout).unwrap();
            println!("output is {:?}", output);
            output
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect_vec()
        })
        .flatten()
        .unique()
        .collect();

    println!("isbns {:?}", isbns);
    let books = isbns
        .iter()
        .map(|isbn| babelio::get_book_metadata_from_isbn(&isbn));
    let books_titles = books
        .clone()
        .map(|b| format!("{} de {}", b.title.unwrap(), b.author.unwrap()))
        .join("\n");
    let blurbs = books.clone().map(|b| b.blurb.unwrap()).join("\n");
    let keywords = books.flat_map(|b| b.key_words.unwrap()).unique().join(", ");

    let custom_message = "custom_message";
    println!("{books_titles}\n\nRésumé:\n{blurbs}\n\n{custom_message}\n\nMots-clés:\n{keywords}")
    /*  let isbn = "9782757862582";
    let isbn = "9782266023160";
    let isbn = "9782253029854";
    let isbn = "9782277223634";
    let isbn = "9782266026659";

    let md = babelio::get_book_metadata_from_isbn(isbn);
    // let md = google_books::get_book_metadata_from_isbn(isbn);
    println!("md {:?}", md) */
}
