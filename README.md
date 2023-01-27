# Book Metadata Finder

Book Metadata Finder find book Metadata on publicly available book database from the bar code.

Metadata include:
- Title
- Author
- Blurb. A book blurb is a short promotional description, whereas a synopsis summarizes the twists, turns, and conclusion of the story.
- Keywords or genres

## Example using Babelio as source
### Input

```rust
let isbn = 9782266071529;
```

### Output
```rust
BookMetaData {
  title: "Le nom de la bête",
  author: {
    surname: "Daniel",
    name: "Easterman",
  },
  blurb: "Janvier 1999. Peu à peu, les pays arabes ont sombré dans l'intégrisme. Les attentats terroristes se multiplient en Europe attisant la haine et le racisme. Au Caire, un coup d'état fomenté par les fondamentalistes permet à leur chef Al-Kourtoubi de s'installer au pouvoir et d'instaurer la terreur. Le réseau des agents secrets britanniques en Égypte ayant été anéanti, Michael Hunt est obligé de reprendre du service pour enquêter sur place. Aidé par son frère Paul, prêtre catholique et agent du Vatican, il apprend que le Pape doit se rendre à Jérusalem pour participer à une conférence œcuménique. Au courant de ce projet, le chef des fondamentalistes a prévu d'enlever le saint père.Dans ce récit efficace et à l'action soutenue, le héros lutte presque seul contre des groupes fanatiques puissants et sans grand espoir de réussir. Comme dans tous ses autres livres, Daniel Easterman, spécialiste de l'islam, part du constat que le Mal est puissant et il dénonce l'intolérance et les nationalismes qui engendrent violence et chaos.--Claude Mesplède<br>\t\t",
  key_words: [
    "roman", "fantastique", "policier historique", "romans policiers et polars", "thriller", "terreur", "action", "démocratie", "mystique", "islam", "intégrisme religieux", "catholicisme", "religion", "terrorisme", "extrémisme", "egypte", "médias", "thriller religieux", "littérature irlandaise", "irlande"
  ],
}
```

## Sources

| Source                                        | Metadata                      | Notes                                                                                                                                                            |
|-----------------------------------------------|-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [Babelio](https://www.babelio.com/)           | title, author, blurb, keyword | No API available. No plan to build one.<br/>Babelio seem to block the IP if it detect this bot is doing some scrapping                                           |
| [GoodReads](https://www.goodreads.com/)       | title, author, blurb, keyword | An API was available, but GoodRead does not create new developer key. [See this](https://help.goodreads.com/s/article/Does-Goodreads-support-the-use-of-APIs)    |
| [Google Books](https://www.google.fr/books/)  | title, author, blurb          | [A real API](https://developers.google.com/books/docs/overview) is available to look up a book by ISBN <br/> Some book can't be search by ISBN, even though a search by title can find them, and they display the right ISBN |
| [ISBSearcher](https://www.isbnsearcher.com/)  | title, author, blurb          |                                                                                                                                                                  |
| [Label Emmaus](https://www.label-emmaus.co/)  | title, author, blurb, genres  |                                                                                                                                                                  |
