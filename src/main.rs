mod parser;
mod request;

// Possible data source
// https://www.babelio.com/      -> title, author, blurb and keyword
// https://www.isbnsearcher.com/ -> title, author, blurb

fn get_book_metadata_from_url(client: &reqwest::blocking::Client, url: String) -> BookMetaData {
    let book_page = request::get_book_page(client, url);
    let id_obj = parser::extract_id_obj(book_page);
    let blurb = request::get_book_blurb_see_more(client, &id_obj);

    BookMetaData {
        blurb,
        title: String::from(""),
        author: String::from(""),
        key_words: vec![],
    }
}

#[derive(Debug)]
struct BookMetaData {
    title: String,
    author: String,
    // A book blurb is a short promotional description.
    // A synopsis summarizes the twists, turns, and conclusion of the story.
    blurb: String,
    key_words: Vec<String>,
}

fn main() {
    let client = reqwest::blocking::Client::builder().build().unwrap();

    let book_url = request::get_book_url(&client, "9782266071529");

    println!("book_url {:#?}", book_url);

    let metadata = get_book_metadata_from_url(&client, book_url);

    println!("BookMetaData {:#?}", metadata)
}

/*

curl 'https://www.babelio.com/aj_recherche.php' \
    --data-raw '{"isMobile":false,"term":"9782266071529"}'


curl 'https://www.babelio.com/aj_recherche.php' \
-H 'Content-Type: application/json' \
--data-raw '{"isMobile":false,"term":"9782266071529"}' \
--compressed
[{"id_oeuvre":"81448","titre":"Le nom de la b\u00eate","couverture":"\/couv\/cvt_Le-nom-de-la-bete_8671.jpg","id":"9555","id_auteur":"9555","prenoms":"Daniel","nom":"Easterman",
"ca_copies":"151","ca_note":"3.42","id_edition":"92919","type":"livres","url":"\/livres\/Easterman-Le-nom-de-la-bete\/81448"}]⏎

*/
