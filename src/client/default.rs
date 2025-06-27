use reqwest::{Client, redirect::Policy};

use crate::{
    client::SearchBy,
    core::{Response, model::Subtitle},
};

pub async fn search(search_by: SearchBy<'_>) -> crate::Result<Response> {
    let mut url: String = search_by.as_ref().into();
    let filter = search_by.filter();
    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        Subtitle::process_url(&mut url, filter);

        let response = client.get(&url).send().await?;

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
            }
        } else {
            return Response::create(&url, &response.text().await?, filter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::search;
    use crate::{
        client::{OrderBy, SearchBy},
        core::Response,
    };

    #[tokio::test]
    async fn test_fetch_url_async() {
        let result = search(SearchBy::MovieAndFilter(
            "the godfather",
            crate::Filters::default()
                .year(1972)
                .languages(&[crate::Language::English])
                .page(2)
                .order_by(OrderBy::Downloads)
                .build(),
        ))
        .await;

        println!("Movies {:#?}", result.as_ref().unwrap());

        if let Ok(Response::Movie(movies)) = &result {
            let sub = search(SearchBy::Url(&movies[0].subtitles_link)).await;
            println!("Subs {:#?}", sub.unwrap());
        }
    }
}
