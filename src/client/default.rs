use reqwest::{Client, redirect::Policy};

use crate::{Result, subtitle::Response};

pub async fn fetch_url() -> Result<Response> {
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+godfather+1972&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+holdovers+2023&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();
    //     https://www.opensubtitles.org/en/search2?MovieName=the godfather 1972&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl&Season=&Episode=&SubSumCD=&Genre=&MovieByteSize=&MovieLanguage=&MovieImdbRatingSign=1&MovieImdbRating=&MovieCountry=&MovieYearSign=1&MovieYear=&MovieFPS=&SubFormat=&SubAddDate=&Uploader=&IDUser=&Translator=&IMDBID=&MovieHash=&IDMovie=
    let mut url =
        "https://www.opensubtitles.org/en/search/sublanguageid-all/idmovie-1196".to_string();

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
        println!("{:#?}", result.unwrap());
    }
}
