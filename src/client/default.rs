use reqwest::{Client, header::USER_AGENT, redirect::Policy};

use crate::{
    client::SearchBy,
    core::{Response, model::Subtitle},
};

/// Performs a search using the provided [`SearchBy`] criteria, handling manual HTTP redirections.
///
/// This function constructs a search URL from the given `search_by` parameter,
/// applies any necessary filters, and performs an HTTP GET request using a [`reqwest::Client`]
/// with redirection disabled. If the response is a redirection, it follows the `Location` header
/// manually. Otherwise, it processes the response and returns a [`Response`].
///
/// # Arguments
///
/// * `search_by` - The search criteria, implementing [`SearchBy`].
///
/// # Returns
///
/// Returns a [`opensubs::Result<Response>`] containing the processed response on success,
/// or an error if any step fails (e.g., network error, invalid header, etc.).
///
/// # Errors
///
/// Returns an error if:
/// - The HTTP client cannot be built.
/// - The HTTP request fails.
/// - The `Location` header in a redirection cannot be parsed.
/// - Response processing fails.
///
/// # Example
///
/// ```
/// use opensubs::{search, SearchBy};
///
/// async fn some() {
///     let result = search(SearchBy::Movie("the godfather")).await.expect("error");
/// }
/// // handle result
/// ```
///
/// # Notes
///
/// - Redirections are followed manually (not automatically by reqwest).
/// - The loop continues following redirects until a non-redirection response is received.
pub async fn search(search_by: SearchBy<'_>) -> crate::Result<Response> {
    let mut url: String = search_by.as_ref().into();
    let filter = search_by.filter();
    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        Subtitle::process_url(&mut url, filter);

        let response = client
            .get(&url)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .send()
            .await?;

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

        assert!(result.is_ok());
        assert!(matches!(result.as_ref().unwrap(), Response::Movie(_)));

        if let Ok(Response::Movie(movies)) = &result {
            let sub = search(SearchBy::Url(&movies[0].subtitles_link)).await;
            assert!(sub.is_ok());
            assert!(matches!(sub.as_ref().unwrap(), Response::Subtitle(_, _)));
        }
    }
}
