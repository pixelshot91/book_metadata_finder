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

    let d_bio = res
        .next()
        .expect(format!("There should be exactly one element with id 'd_bio', html {:?}", html).as_str());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_id_obj_from_file() {
        let html = std::fs::read_to_string("src/babelio/test/get_book.html").unwrap();
        let id_obj = extract_id_obj(html);
        assert_eq!(id_obj, "827593");
    }
}
