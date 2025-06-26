use reqwest::{blocking::Client, redirect::Policy};

use crate::{client::SearchBy, core::Response};

pub fn search(search_by: SearchBy<'_>) -> crate::Result<Response> {
    let search_by = &search_by;
    let mut url: String = search_by.into();

    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        let response = client.get(&url).send()?;

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
            }
        } else {
            let filter = match search_by {
                SearchBy::MovieAndFilter(_, filter) => Some(filter),
                _ => None,
            };

            return Response::create(&url, &response.text()?, filter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::search;
    use crate::SearchBy;

    #[test]
    fn test_fetch_url() {
        let result = search(SearchBy::MovieAndFilter(
            "holdovers",
            crate::Filters::default()
                .languages(&[crate::Language::Spanish])
                .build(),
        ));

        //         println!("Subs {:#?}", result.unwrap());
    }
}
