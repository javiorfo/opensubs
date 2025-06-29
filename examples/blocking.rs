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
