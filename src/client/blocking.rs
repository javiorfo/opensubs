use reqwest::{blocking::Client, redirect::Policy};

use crate::Result;

pub fn fetch_url() -> Result<String> {
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+godfather+1972&id=8&action=search&SubLanguageID=eng&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl,eng".to_string();
    let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+holdovers+2023&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();

    let client = Client::builder().redirect(Policy::none()).build()?;

    loop {
        println!("Requesting URL: {}", url);

        let response = client.get(&url).send()?;

        println!("Status: {}", response.status());

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
                println!("Redirecting to: {}", url);
            }
        } else {
            return Ok(response.text()?);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_url;

    #[test]
    fn test_fetch_url() {
        let result = fetch_url();
        assert!(result.is_ok());
        let body = result;
        assert!(body.is_ok());
    }
}
