use reqwest::{Client, redirect::Policy};

use crate::{client::SearchBy, core::Response};

pub async fn search(search_by: SearchBy<'_>) -> crate::Result<Response> {
    let search_by = &search_by;
    let mut url: String = search_by.into();

    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        println!("Requesting URL: {}", url);

        let response = client.get(&url).send().await?;

        println!("Status: {}", response.status());

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
                println!("Redirecting to: {}", url);
            }
        } else {
            let filter = match search_by {
                SearchBy::MovieAndFilter(_, filter) => Some(filter),
                _ => None,
            };

            return Response::create(&url, &response.text().await?, filter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::search;
    use crate::{client::SearchBy, core::Response};

    #[tokio::test]
    async fn test_fetch_url_async() {
        let result = search(SearchBy::MovieAndFilter(
            "the godfather",
            crate::Filters::default()
                .year(1972)
                .languages(&[crate::Language::Spanish])
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
