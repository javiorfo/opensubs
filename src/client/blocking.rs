use reqwest::{blocking::Client, redirect::Policy};

use crate::{
    client::SearchBy,
    core::{Response, model::Subtitle},
};

#[allow(dead_code)]
pub fn search(search_by: SearchBy) -> crate::Result<Response> {
    let mut url: String = search_by.as_ref().into();
    let filter = search_by.filter();
    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        Subtitle::process_url(&mut url, filter);

        let response = client.get(&url).send()?;

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
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
    fn test_fetch_url() {
        let result = search(SearchBy::MovieAndFilter(
            "holdovers",
            crate::Filters::default()
                .languages(&[crate::Language::Spanish, crate::Language::SpanishLA])
                .build(),
        ));

        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Response::Subtitle(_, _)));
    }
}
