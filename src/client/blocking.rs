use reqwest::{blocking::Client, header::USER_AGENT, redirect::Policy};

use crate::{
    client::SearchBy,
    core::{Response, model::Subtitle},
};

/// Performs a synchronous search using the provided [`SearchBy`] criteria, handling HTTP redirections manually.
///
/// This function builds a search URL from the given `search_by` parameter, applies any necessary filters,
/// and sends a synchronous HTTP GET request using a [`reqwest::blocking::Client`] with redirection disabled.
/// If the response is a redirection, it follows the `Location` header manually in a loop until a non-redirection
/// response is received. The final response is processed and returned as a [`Response`].
///
/// # Arguments
///
/// * `search_by` - Search criteria implementing [`SearchBy`].
///
/// # Returns
///
/// Returns a [`opensubs::Result<Response>`] containing the processed response on success, or an error if any step fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP client cannot be built.
/// - The HTTP request fails.
/// - The `Location` header in a redirection cannot be parsed as a valid string.
/// - Response processing fails.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "blocking")]
/// # {
/// use opensubs::{blocking, SearchBy};
///
/// let result = blocking::search(SearchBy::Movie("the godfather")).expect("error");
/// // handle result
/// # }
/// ```
///
/// # Notes
///
/// - Redirections are handled manually (not automatically by reqwest).
/// - The function loops, following redirects, until a non-redirection response is received.
/// - This is a blocking synchronous version (available by cargo feature "blocking")
#[allow(dead_code)]
pub fn search(search_by: SearchBy) -> crate::Result<Response> {
    let mut url: String = search_by.as_ref().into();
    let filter = search_by.filter();
    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        Subtitle::process_url(&mut url, filter);

        let response = client
            .get(&url)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .send()?;

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
                if !url.contains("www.opensubtitles.org") {
                    url = format!("https://www.opensubtitles.org{}", url);
                }
            }
        } else {
            return Response::create(&url, &response.text()?, filter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::search;
    use crate::{Response, SearchBy};

    #[test]
    fn test_search_by_movie_and_filter() {
        let result = search(SearchBy::MovieAndFilter(
            "81/2",
            crate::Filters::default()
                .languages(&[crate::Language::Spanish, crate::Language::SpanishLA])
                .build(),
        ));

        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Response::Movie(_)));
    }

    #[test]
    fn test_search_by_movie() {
        let result = search(SearchBy::MovieAndFilter(
            "pulp fiction",
            crate::Filters::default()
                .languages(&[crate::Language::Spanish, crate::Language::SpanishLA])
                .build(),
        ));

        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Response::Subtitle(_, _)));
    }
}
