use reqwest::{Client, redirect::Policy};

use crate::{subtitle::Response, Result};

pub async fn fetch_url() -> Result<Response> {
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+godfather+1972&id=8&action=search&SubLanguageID=eng&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl,eng".to_string();
    let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+holdovers+2023&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();

    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        println!("Requesting URL: {}", url);

        let response = client.get(&url).send().await?;

        println!("Status: {}", response.status());

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
                // si contiene imdbid va a los subs directamente, sino listado de posible matches
                println!("Redirecting to: {}", url);
            } else {
                println!("Redirect status but no Location header");
            }
        } else {
            return Ok(Response::create(&url, &response.text().await?).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_url;

    #[tokio::test]
    async fn test_fetch_url_async() {
        let result = fetch_url().await;
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
    }
}
