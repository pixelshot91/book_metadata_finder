#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

pub fn get_book_url(client: &reqwest::blocking::Client, isbn: &str) -> String {
    let resp = client
        .post("https://www.babelio.com/aj_recherche.php")
        .body(format!("{{\"isMobile\":false,\"term\":\"{}\"}}", isbn))
        .send()
        .unwrap();
    let r = resp.json::<Vec<BabelioISBNResponse>>().unwrap();
    r[0].url.clone()
}

pub fn get_book_page(client: &reqwest::blocking::Client, url: String) -> String {
    let resp = client
        .get(format!("https://www.babelio.com{url}"))
        .send()
        .unwrap();
    resp.text().unwrap()
}

pub fn get_book_blurb_see_more(client: &reqwest::blocking::Client, id_obj: &str) -> String {
    let params = std::collections::HashMap::from([("type", "1"), ("id_obj", id_obj)]);

    let voir_plus_resp = client
        .post("https://www.babelio.com/aj_voir_plus_a.php")
        .form(&params)
        .send()
        .unwrap();
    voir_plus_resp.text().unwrap()
}
