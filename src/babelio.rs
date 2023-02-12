use crate::{cached_client::CachedClient, common};
mod parser;
mod request;

pub fn get_book_metadata_from_isbn(isbn: &str) -> common::BookMetaData {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let cached_client = CachedClient {
        http_client: client,
    };
    let book_url = request::get_book_url(&cached_client, isbn);
    let book_page = request::get_book_page(&cached_client, book_url);
    let blurb_res = parser::extract_blurb(&book_page);

    let raw_blurb = match blurb_res {
        parser::BlurbRes::SmallBlurb(blurb) => blurb,
        parser::BlurbRes::BigBlurb(id_obj) => {
            request::get_book_blurb_see_more(&cached_client, &id_obj)
        }
    };

    let mut res = parser::extract_title_author_keywords(&book_page);
    res.blurb = parser::parse_blurb(&raw_blurb);
    res
}

#[cfg(test)]
mod tests {
    use crate::common::BookMetaData;

    use super::*;

    #[test]
    fn extract_id_obj_from_file() {
        let isbn = "9782266172363";
        let md = get_book_metadata_from_isbn(isbn);
        assert_eq!(md, BookMetaData {
            title: Some("Le nom de la bête".to_string()),
            author: Some("Daniel Easterman".to_string()),
            blurb: Some("Saint-Libéral, petit village de Corrèze, début du XXe siècle. Dans la neige crissent trois paires de sabots. Les enfants relèvent les collets. Sept grives ! C'est un beau butin. Qu'à l'approche des loups il faudra leur abandonner... Ce qu'il faut laisser derrière soi, déjà, pour survivre ! <br>Plus tard, faute de grives, ils sacrifieront les coutumes d'antan, un savoir-faire dépassé, un père, une mère, une terre. Nous sommes en 1914, et les loups ont passé les Vosges...<br>		".to_string()),
            key_words: Some(
                [
                    "roman",
                    "fantastique",
                    "policier historique",
                    "romans policiers et polars",
                    "thriller",
                    "terreur",
                    "action",
                    "démocratie",
                    "mystique",
                    "islam",
                    "intégrisme religieux",
                    "catholicisme",
                    "religion",
                    "terrorisme",
                    "extrémisme",
                    "egypte",
                    "médias",
                    "thriller religieux",
                    "littérature irlandaise",
                    "irlande"
                ]
                .map(|s| s.to_string())
                .to_vec()
            ),
        });
    }
}
