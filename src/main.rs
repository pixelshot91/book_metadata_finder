use std::fs;

use regex::Regex;
use scraper::{Html, Selector};
// use html_parser::Dom;
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

fn get_book_metadata_from_url(client: reqwest::blocking::Client, url: String) {
    /* let resp = client
    .get(format!("https://www.babelio.com/{url}"))
    .send()
    .unwrap(); */
    let html = fs::read_to_string("test/babelio_response.html").unwrap();
    let doc = Html::parse_document(html.as_str());

    let selector = Selector::parse("#d_bio").expect("Response should contain a element whose id is 'd_bio'");
    let mut res = doc.select(&selector);

    let d_bio = res
        .next()
        .expect("There should be exactly one element with id 'd_bio'");
    let span = d_bio
        .children()
        .nth(1)
        .expect("d_bio second child should be a span")
        .children()
        .nth(1)
        .expect("style span should have a second child");
    let onclick = span
        .value()
        .as_element()
        .expect("style span second child should be a <a href ...>")
        .attr("onclick")
        .expect("<a href ...> should have a 'onclick' attribute");

    println!("onclick {:?}", onclick);

    let re = Regex::new(r"javascript:voir_plus_a\('#d_bio',1,(\d+)\);").unwrap();

    let single_capture = re.captures_iter(onclick).next().expect("The onclick should match with the regex");
    let id_obj = &single_capture[1];
    println!("id_obj {:?}", id_obj);
    // println!("found {} captures", re.captures_len());
    /* for cap in re.captures_iter(onclick) {
        println!("cap {}", &cap[1]);
    } */


    /* let voir_plus_id = "type:
    1
    id_obj:
    827593"; */

    /*
    let mut params = std::collections::HashMap::from(
        [("type", "1"), ("id_obj", "827593")]
    );

    let voir_plus_resp = client.post("https://www.babelio.com/aj_voir_plus_a.php").form(&params).send().unwrap();

    let blurb = voir_plus_resp.text().unwrap();
    */

    // assert!(Dom::parse(html.as_str()).is_ok());
}

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

    /*let book_url = get_book_url(client, "9782266071529");

    println!("book_url {:#?}", book_url); */

    get_book_metadata_from_url(client, String::from("dfds"));
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
