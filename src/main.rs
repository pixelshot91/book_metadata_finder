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

    let book_metadata_providers: Vec<Box<dyn common::Provider>> = vec![
        Box::new(babelio::Babelio {}),
        Box::new(google_books::GoogleBooks {}),
    ];

    let books: Vec<common::BookMetaData> = isbns
        .iter()
        .map(|isbn| {
            for provider in &book_metadata_providers {
                let res = provider.get_book_metadata_from_isbn(&isbn);
                if let Some(r) = res {
                    return r;
                }
            }
            panic!("No provider find any information on book {}", isbn)
            /* book_metadata_providers[0]
            .get_book_metadata_from_isbn(&isbn)
            .unwrap() */
        })
        .collect();
    let books_titles = books
        .iter()
        .map(|b| {
            format!(
                "\"{}\" {}",
                b.title,
                vec_fmt(
                    b.authors
                        .iter()
                        .map(|a| format!("{} {}", a.first_name, a.last_name))
                        .collect_vec()
                )
            )
        })
        .join("\n");
    let blurbs = books
        .iter()
        .map(|b| format!("{}:\n{}\n", b.title, b.blurb.as_ref().unwrap()))
        .join("\n");
    let keywords = books.iter().flat_map(|b| &b.keywords).unique().join(", ");

    let custom_message = std::fs::read_to_string("custom_message.txt").unwrap();

    let mut ad = books_titles + "\n\nRésumé:\n" + &blurbs + "\n" + &custom_message;
    if !keywords.is_empty() {
        ad = ad + "\nMots-clés:\n" + &keywords;
    }

    println!("{}", ad);
}

fn vec_fmt(vec: Vec<String>) -> String {
    match vec.len() {
        0 => "".to_string(),
        1 => format!("de {}", vec[0]),
        2 => format!("de {} et {}", vec[0], vec[1]),
        _ => panic!("More than 2 authors"),
    }
}
