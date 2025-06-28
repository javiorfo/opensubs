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
