use crate::cached_client::CachedClient;

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

pub fn get_book_url(client: &CachedClient, isbn: &str) -> String {
    client.get_from_cache("cache/babelio/get_book_url.html", |http_client| {
        let resp = http_client
            .post("https://www.babelio.com/aj_recherche.php")
            .body(format!("{{\"isMobile\":false,\"term\":\"{}\"}}", isbn))
            .send()
            .unwrap();
        let r = resp.json::<Vec<BabelioISBNResponse>>().unwrap();
        r[0].url.clone()
    })
}

pub fn get_book_page(client: &CachedClient, url: String) -> String {
    client.get_from_cache("cache/babelio/get_book_page.html", |http_client| {
        let resp = http_client
            .get(format!("https://www.babelio.com{url}"))
            .send()
            .unwrap();
        resp.text().unwrap()
    })
}

pub fn get_book_blurb_see_more(client: &CachedClient, id_obj: &str) -> String {
    client.get_from_cache(
        "cache/babelio/get_book_blurb_see_more.html",
        |http_client| {
            let params = std::collections::HashMap::from([("type", "1"), ("id_obj", id_obj)]);

            let voir_plus_resp = http_client
                .post("https://www.babelio.com/aj_voir_plus_a.php")
                .form(&params)
                .send()
                .unwrap();
            voir_plus_resp.text().unwrap()
        },
    )
}
