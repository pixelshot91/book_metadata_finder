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
        .map(|b| {
            format!(
                "{} {}",
                b.title.unwrap(),
                vec_fmt(
                    b.authors
                        .unwrap()
                        .iter()
                        .map(|a| format!("{} {}", a.first_name, a.last_name))
                        .collect_vec()
                )
            )
        })
        .join("\n");
    let blurbs = books
        .clone()
        .map(|b| format!("{}:\n{}\n", b.title.unwrap(), b.blurb.unwrap()))
        .join("\n");
    let keywords = books.flat_map(|b| b.key_words.unwrap()).unique().join(", ");

    let custom_message = std::fs::read_to_string("custom_message.txt").unwrap();
    println!("{books_titles}\n\nRésumé:\n{blurbs}\n{custom_message}\nMots-clés:\n{keywords}")
}

fn vec_fmt(vec: Vec<String>) -> String {
    match vec.len() {
        0 => "".to_string(),
        1 => format!("de {}", vec[0]),
        2 => format!("de {} et {}", vec[0], vec[1]),
        _ => panic!("More than 2 authors"),
    }
}
