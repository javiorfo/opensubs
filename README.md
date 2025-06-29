# opensubs
Library to search subtitles from opensubtitles.org 

This crate provides a high-level, ergonomic API for searching and retrieving subtitles and related metadata
from `opensubtitles.org`. It offers both asynchronous and blocking (synchronous) interfaces, with flexible
filtering and ordering options.

## Usage
Add this crate to your `Cargo.toml`:

```toml
[dependencies]
opensubs = "0.1.0"
```

#### Enable blocking feature if needed

```
[dependencies]
opensubs = { version = "0.1.0", features = ["blocking"] }
```

## Async Example (default)

```rust
use opensubs::{Filters, Language, OrderBy, SearchBy};

#[tokio::main]
async fn main() -> opensubs::Result {
    // async search movie "holdovers", spanish subs, order by rating
    let results = opensubs::search(SearchBy::MovieAndFilter(
        "holdovers",
        Filters::default()
            .languages(&[Language::Spanish])
            .order_by(OrderBy::Rating)
            .build(),
    ))
    .await?;

    println!("Subtitles {results:#?}");

    Ok(())
}
```

## Blocking Example (feature "blocking")

```rust
use opensubs::{Filters, Language, OrderBy, Response, SearchBy};

fn main() -> opensubs::Result {
    // blocking search movie "the godfather"
    // year 1972, french and german subs, order by rating
    let results = opensubs::blocking::search(SearchBy::MovieAndFilter(
        "the godfather",
        Filters::default()
            .year(1972)
            .languages(&[Language::French, Language::German])
            .order_by(OrderBy::Downloads)
            .build(),
    ))?;

    match results {
        Response::Movie(movies) => {
            // If results is Movie type, get the subtitles_link property 
            // and find subtitles for it
            if let Some(movie) = movies.first() {
                let subs = opensubs::blocking::search(SearchBy::Url(&movie.subtitles_link))?;
                println!("Subtitles {subs:#?}");
            }
        }
        // else print the subtitles
        _ => println!("Subtitles {results:#?}"),
    }

    Ok(())
}
```

## Details
- Searching subtitles from `opensubtitles.org` could return a list of movies or a list of subtitles of the movie searched (if the text and filter are more exactly). For that matter the [Response](https://github.com/javiorfo/opensubs/blob/736b5a0d68fd2c7622bc1426458b204f7b3daf96/src/core/response.rs#L53) is an enum.
- Here are more [examples](https://github.com/javiorfo/opensubs/tree/master/examples)

## Features
- Default async search. Blocking search available too
- Search by url, movie name and/or filters (languages, page, ordering and year)
- Obtain not only info and metadata but also a subtitle download link. [Here](https://github.com/javiorfo/opensubs/blob/master/examples/download_sub.rs) is an example of download using `wget`

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G)
