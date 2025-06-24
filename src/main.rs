use reqwest::{Client, redirect::Policy};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+godfather+1972&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();
    //     let mut url = "https://www.opensubtitles.org/en/search2?MovieName=the+holdovers+2023&id=8&action=search&SubLanguageID=spa&SubLanguageID=spl&SubLanguageID=spa,spl".to_string();

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

            let table_selector = Selector::parse("table#search_results").unwrap();

            let line_selector = Selector::parse("tr").unwrap();
            let column_selector = Selector::parse("td").unwrap();

            if let Some(table) = document.select(&table_selector).next() {
                // skip 1 (table header)
                for line in table.select(&line_selector).skip(1) {
                    let id = match line.attr("id") {
                        // Omit non-display items
                        Some(id) if !id.contains("ihtr") => id.strip_prefix("name").unwrap_or(id),
                        _ => continue,
                    };

                    let movie_name = line
                        .text()
                        .take(2) // Omit links in movie name
                        .filter(|text| !text.contains("Watch online"))
                        .collect::<Vec<_>>();

                    println!("id {} Movie: {:?}", id, movie_name);

                    // skip 1 (movie name and links)
                    let mut data = line.select(&column_selector).skip(1);
                    let lang = data
                        .next()
                        .and_then(|column| {
                            column
                                .first_child()
                                .and_then(|child| child.value().as_element())
                                .and_then(|element| element.attr("title"))
                        })
                        .unwrap_or("Not Available");
                    println!("language: {}", lang);

                    let cd = match data.next() {
                        Some(column) => column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string(),
                        None => "".to_string(),
                    };
                    println!("cd: {}", cd);

                    let date = match data.next() {
                        Some(column) => column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string(),
                        None => "".to_string(),
                    };
                    println!("date: {}", date);

                    let downloads = match data.next() {
                        Some(column) => column
                            .text()
                            .collect::<Vec<_>>()
                            .first()
                            .unwrap()
                            .to_string()
                            .replace("x", ""),
                        None => "".to_string(),
                    };
                    println!("downloads: {}", downloads);

                    let uploader = data.nth(3).and_then(|column| {
                        let name = column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        if name.is_empty() { None } else { Some(name) }
                    });
                    println!("uploader: {:?}", uploader);

                }
            }
            break;
        }
    }

    Ok(())
}

// link descarga
// "https://dl.opensubtitles.org/en/download/sub/" + code
