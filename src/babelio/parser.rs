use crate::common::{html_select, BookMetaData};
use itertools::Itertools;

pub fn extract_id_obj(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);

    let selector = scraper::Selector::parse("#d_bio").expect(
        format!(
            "Response should contain a element whose id is 'd_bio', html is {:?}",
            html
        )
        .as_str(),
    );
    let mut res = doc.select(&selector);

    let d_bio = res.next().expect(
        format!(
            "There should be exactly one element with id 'd_bio', html {:?}",
            html
        )
        .as_str(),
    );
    let span = d_bio
        .children()
        .rev().nth(1)
        .expect("d_bio should have a second to last children (the style span)")
        .children()
        .nth(1)
        .expect("d_bio second child should be a style span which should have a second child (the onclick)");
    let onclick = span
        .value()
        .as_element()
        .expect("style span second child should be a <a href ...>")
        .attr("onclick")
        .expect("<a href ...> should have a 'onclick' attribute");

    let re = regex::Regex::new(r"javascript:voir_plus_a\('#d_bio',1,(\d+)\);").unwrap();

    let single_capture = re
        .captures_iter(onclick)
        .next()
        .expect("The onclick should match with the regex");
    let id_obj = &single_capture[1];
    String::from(id_obj)
}

pub fn extract_title_author_keywords(html: &str) -> BookMetaData {
    let doc = scraper::Html::parse_document(html);

    let book_select = html_select("div[itemscope][itemtype=\"https://schema.org/Book\"]");
    let res = doc.select(&book_select);
    let book_scope = res.exactly_one().expect(format!(
        "Response should contain a element whose with id is itemscope and itemtype=\"https://schema.org/Book\", html is {:?}",
        html
    )
    .as_str());
    let title_select = html_select("[itemprop=\"name\"]");
    let mut res2 = book_scope.select(&title_select).into_iter();
    let title = res2
        .next()
        .expect("There should be at least one element with itemprop=\"name\"")
        .first_child()
        .unwrap()
        .first_child()
        .unwrap()
        .value()
        .as_text()
        .unwrap()
        .trim()
        .to_string();

    let binding =
        html_select("[itemprop=\"author\"][itemscope][itemtype=\"https://schema.org/Person\"]");
    let r = book_scope.select(&binding);

    let author_scope = r.exactly_one().expect(
            "Response should contain a element whose itemprop=\"author\" and itemscope and itemtype=\"https://schema.org/Person\""
        );

    let author_span = author_scope
        .first_child()
        .expect("author_scope shoud have a first child <a ...>")
        .first_child()
        .expect("author scope > a shoud have a first child <span ...>");
    let author_first_name = author_span
        .first_child()
        .expect("author scope > a > span shoud have a first child which is first name")
        .value()
        .as_text()
        .expect("should be a text")
        .to_string();
    let author_last_name = author_span
        .children()
        .nth(1)
        .expect("author scope > a > span shoud have a second child which is the last name")
        .first_child()
        .unwrap()
        .value()
        .as_text()
        .expect("should be a text")
        .to_string();

    let keywords_scope = book_scope
        .select(&html_select("[class=\"tags\"]"))
        .exactly_one()
        .unwrap();
    let keywords = keywords_scope
        .children()
        .filter_map(|c| {
            Some(
                c.first_child()?
                    .value()
                    .as_text()
                    .expect("c should be a text")
                    .trim()
                    .to_string(),
            )
        })
        .collect();
    BookMetaData {
        title: Some(title),
        author: Some(format!(
            "{} {}",
            author_first_name.trim(),
            author_last_name.trim()
        )),
        key_words: Some(keywords),
        ..Default::default()
    }
}

pub fn parse_blurb(raw_blurb: &str) -> Option<String> {
    Some(raw_blurb.replace("<br>", "\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_id_obj_from_file() {
        let html = std::fs::read_to_string("src/babelio/test/get_book.html").unwrap();
        let id_obj = extract_id_obj(&html);
        assert_eq!(id_obj, "827593");
    }
    #[test]
    pub fn extract_title_author_keywords_from_file() {
        let html = std::fs::read_to_string("src/babelio/test/get_book_minimal.html").unwrap();
        let title_author_keywords = extract_title_author_keywords(&html);
        assert_eq!(
            title_author_keywords,
            BookMetaData {
                title: Some("Le nom de la bête".to_string()),
                author: Some("Daniel Easterman".to_string()),
                blurb: None,
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
            }
        );
    }
}
