use serde::{Deserialize, Serialize};

fn get_book_url(client: reqwest::blocking::Client, isbn: &str) -> String {
    let resp = client
        .post("https://www.babelio.com/aj_recherche.php")
        .body("{\"isMobile\":false,\"term\":\"9782266071529\"}")
        .send()
        .unwrap();
    let r = resp.json::<Vec<BabelioISBNResponse>>().unwrap();
    r[0].url.clone()
}

// fn get_book_metadata_from_url(client: reqwest::blocking::Client, url: String) {
//     let resp = client
//         .get(format!("https://www.babelio.com/{url}"))
//         .send()
//         .unwrap();
    
// }

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

    let book_url = get_book_url(client, "9782266071529");

    println!("book_url {:#?}", book_url);
}

#[derive(Serialize, Deserialize, Debug)]
struct BabelioISBNResponse {
    id_oeuvre: String,
    titre: String,
    couverture: String,
    id: String,
    id_auteur: String,
    prenoms: String,
    nom: String,
    ca_copies: String,
    ca_note: String,
    id_edition: String,
    r#type: String,
    url: String,
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
