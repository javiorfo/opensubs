use reqwest::{Client, redirect::Policy};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+godfather+1972&id=8&action=search&SubLanguageID=eng&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl,eng".to_string();
    let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+holdovers+2023&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();

    let client = Client::builder()
        .redirect(Policy::none())
        .build()?;

    loop {
        println!("Requesting URL: {}", url);

        let response = client
            .get(&url)
            .send()
            .await?;

        println!("Status: {}", response.status());

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                url = location.to_str()?.to_string();
                // si contiene imdbid va a los subs directamente, sino listado de posible matches
                println!("Redirecting to: {}", url);
            } else {
                println!("Redirect status but no Location header");
                break;
            }
        } else {
            let body = response.text().await?;
            let document = Html::parse_document(&body);

            if url.contains("imdbid") {
                let div = Selector::parse("div#msg").unwrap();
                let div = document.select(&div).next().unwrap();
                for d in div.select(&Selector::parse("span").unwrap()).skip(1) {
                    println!("Paginator");
                    println!("{}", d.text().collect::<Vec<_>>().join(" "));
                    println!("++");
                }
            }

            // if Paginator fails
            // get first results id
            // https://www.opensubtitles.org/en/search/sublanguageid-all/idmovie-1196

            let table_selector = Selector::parse("table#search_results").unwrap();

            let tr_selector = Selector::parse("tr").unwrap();
            let td_selector = Selector::parse("td").unwrap();

            if let Some(table) = document.select(&table_selector).next() {
                for tr in table.select(&tr_selector).skip(1) {
                    let header_text = tr
                        .text()
                        .take(2)
                        .filter(|text| !text.contains("Watch online"))
                        .collect::<Vec<_>>()
                        .join("");

                    let id = tr.attr("id").unwrap_or_default();

                    if id.is_empty() || id.contains("ihtr") {
                        continue;
                    }

                    println!("id {} Header: {}", id, header_text);
                    for (i, td) in tr.select(&td_selector).skip(1).enumerate() {
                        if i == 0 {
                            let lang = td.first_child().unwrap().value().as_element().unwrap().attr("title").unwrap();
                            println!("lang: {:?}", lang);
                        }
                        let text = td.text().collect::<Vec<_>>().join(" ");
                        println!("td: {}", text);
                    }
                    println!("++++++++++++++++++++++++++++++++++");
                }
            } else {
                println!("Table with id 'results' not found.");
            }
            break;
        }
    }

    Ok(())
}

// link descarga
// "https://dl.opensubtitles.org/en/download/sub/" + code
