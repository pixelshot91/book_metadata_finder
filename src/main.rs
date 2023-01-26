mod babelio;
mod common;

// Possible data source
// https://www.babelio.com/      -> title, author, blurb and keyword
// https://www.isbnsearcher.com/ -> title, author, blurb



fn main() {
    babelio::get_book_metadata_from_isbn("9782266071529");
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
