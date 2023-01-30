use crate::common::{html_select, BookMetaData};
use itertools::Itertools;

pub fn extract_id_obj(html: String) -> String {
    let doc = scraper::Html::parse_document(html.as_str());

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

    let re = regex::Regex::new(r"javascript:voir_plus_a\('#d_bio',1,(\d+)\);").unwrap();

    let single_capture = re
        .captures_iter(onclick)
        .next()
        .expect("The onclick should match with the regex");
    let id_obj = &single_capture[1];
    String::from(id_obj)
}

fn extract_title_author_keywords(html: &str) -> BookMetaData {
    let doc = scraper::Html::parse_document(html);

    let book_select = html_select("div[itemscope][itemtype=\"https://schema.org/Book\"]");
    let res = doc.select(&book_select);
    let book_scope = res.exactly_one().expect(format!(
        "Response should contain a element whose with id is itemscope and itemtype=\"https://schema.org/Book\", html is {:?}",
        html
    )
    .as_str());
    println!("bookscope {:#?}", book_scope);
    let title_select = html_select("[itemprop=\"name\"]");
    let book_sub_html = scraper::Html::parse_document(&book_scope.html());
    let mut res2 = book_scope.select(&title_select).into_iter();
    /*  let (mut res, res_copy) = res.tee();
    for r in res_copy {
        println!("XXXXXXXXXXXXXXXXXX = {:#?}", r);
    }
    println!("1111111111 = {:?}", res.next());
    println!("222222222 = {:?}", res.next());
    println!("3333333333 = {:?}", res.next());
    println!("444444444444 = {:?}", res.next());
    let mut res = res.peekable();
    // println!("{:?}", res.size_hint());
    println!("{:?}", res.peek());
    let title = res
        .peek() */
    // The iterator from select is circular.
    // The first next does not ouput the first element, but the second one which does not exit, so it returns None
    // I call next first to flush the None.
    // The next None will output the third element, which does not exist, so it ouputs the first element
    // res2.next();
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
    println!("TITLE ========= {:?}", title);

    let binding =
        html_select("[itemprop=\"author\"][itemscope][itemtype=\"https://schema.org/Person\"]");
    let mut r = book_scope.select(&binding);
    /* println!("1111111111 = {:?}", r.next());
    println!("222222222 = {:?}", r.next());
    println!("3333333333 = {:?}", r.next());
    println!("444444444444 = {:?}", r.next()); */
    /* for r in book_scope.select(&html_select("[itemprop=\"author\"][itemscope][itemtype=\"https://schema.org/Person\"]")) {
        println!("AAAAAAAAAAUTHOR r = {:?}", r);
    } */
    // r.next();
    let author_scope = r.exactly_one().expect(format!(
            "Response should contain a element whose itemprop=\"author\" and itemscope and itemtype=\"https://schema.org/Person\", html is {:?}",
            42 //html
        )
        .as_str());

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
    /*dbg!(author_scope.html());
    let author_1 = &author_scope
        .first_child()
        .expect("author scope shoud have a first child <a ...>");
    let author_1_clone = author_1.clone();
    for c in author_1_clone.children() {
        println!("c = {:?}", c.value());
    }
     let author_a = author_1.children().nth(1)
        .expect("author scope > a shoud have a second child of author first name")
        .value();
    dbg!(author_a.as_text());
    let author = author_a
        .as_text()
        .unwrap()
        .to_string();*/
    /*
    let keywords_scope = book_scope
        .select(&html_select("p[class=\"tags\"]"))
        .exactly_one()
        .unwrap();
    let keywords = keywords_scope
        .children()
        .map(|c| c.value().as_text().unwrap().to_string())
        .collect();*/
    BookMetaData {
        title: Some(title),
        author: Some(format!(
            "{} {}",
            author_first_name.trim(),
            author_last_name.trim()
        )),
        // key_words: Some(keywords),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_id_obj_from_file() {
        let html = std::fs::read_to_string("src/babelio/test/get_book.html").unwrap();
        let id_obj = extract_id_obj(html);
        assert_eq!(id_obj, "827593");
    }
    #[test]
    fn extract_title_author_keywords_from_file() {
        let html = std::fs::read_to_string("src/babelio/test/get_book_minimal.html").unwrap();
        let title_author_keywords = extract_title_author_keywords(&html);
        assert_eq!(
            title_author_keywords,
            BookMetaData {
                title: Some("Le nom de la bête".to_string()),
                author: Some("Daniel Easterman".to_string()),
                blurb: None,
                key_words: None,
            }
        );
    }
}
