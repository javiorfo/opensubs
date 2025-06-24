use super::model;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug, Default)]
pub struct Page {
    pub from: u32,
    pub to: u32,
    pub total: u32,
}

impl From<Option<String>> for Page {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(ref text) => {
                let parsed_numbers = Regex::new(r"\d+(\.\d+)?")
                    .expect("Error setting regex")
                    .find_iter(text)
                    .filter_map(|token| token.as_str().parse::<u32>().ok())
                    .collect::<Vec<u32>>();

                if parsed_numbers.len() < 3 {
                    Self::default()
                } else {
                    Self {
                        from: parsed_numbers[0],
                        to: parsed_numbers[1],
                        total: parsed_numbers[2],
                    }
                }
            }
            _ => Self::default(),
        }
    }
}

#[derive(Debug)]
pub enum Response {
    Movie(Vec<model::Movie>),
    Subtitle(Page, Vec<model::Subtitle>),
}

impl Response {
    pub(crate) fn create(url: &str, html: &str) -> crate::Result<Self> {
        let document = Html::parse_document(html);

        let table_selector = Selector::parse("table#search_results")?;
        let line_selector = Selector::parse("tr")?;
        let column_selector = Selector::parse("td")?;

        if url.contains("imdbid") || url.contains("idmovie") {
            let page = match document.select(&Selector::parse("div#msg")?).next() {
                Some(page) => page
                    .select(&Selector::parse("span")?)
                    .nth(1)
                    .map(|page| page.text().collect::<Vec<_>>().join(" "))
                    .into(),
                None => Page::default(),
            };

            let mut subtitles = Vec::new();
            if let Some(table) = document.select(&table_selector).next() {
                // skip 1 (table header)
                for line in table.select(&line_selector).skip(1) {
                    let id = match line.attr("id") {
                        // Omit non-display items
                        Some(id) if !id.contains("ihtr") => id.strip_prefix("name").unwrap_or(id),
                        _ => continue,
                    }
                    .parse()
                    .unwrap_or_default();

                    let movie_text = line
                        .text()
                        .take(2) // Omit links in movie name
                        .filter(|text| !text.contains("Watch online"))
                        .collect::<Vec<_>>();

                    let movie = movie_text
                        .first()
                        .map(|value| value.replace("\n", "").replace("\t", "").to_string())
                        .unwrap_or_default();

                    let name = movie_text
                        .get(1)
                        .map(|value| value.replace("\n", "").replace("\t", "").to_string());

                    // skip 1 (movie name and links)
                    let mut data = line.select(&column_selector).skip(1);
                    let language = data
                        .next()
                        .and_then(|column| {
                            column
                                .first_child()
                                .and_then(|child| child.value().as_element())
                                .and_then(|element| element.attr("title"))
                        })
                        .unwrap_or("Not Available")
                        .to_string();

                    let cd = data
                        .next()
                        .map(|column| {
                            column
                                .text()
                                .collect::<Vec<_>>()
                                .join(" ")
                                .trim()
                                .to_string()
                        })
                        .unwrap_or_default();

                    let uploaded = data
                        .next()
                        .map(|column| {
                            let mut date = column
                                .text()
                                .collect::<Vec<_>>()
                                .join(" ")
                                .trim()
                                .to_string();
                            // Takes only the date format DD/MM/YY
                            date.truncate(8);
                            date
                        })
                        .unwrap_or_default();

                    let downloads: u32 = data
                        .next()
                        .and_then(|column| column.text().next())
                        .map(|text| text.trim().replace("x", ""))
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or_default();

                    let uploader = data.nth(3).and_then(|column| {
                        let name = column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        if name.is_empty() { None } else { Some(name) }
                    });

                    subtitles.push(model::Subtitle::new(
                        id, movie, name, language, cd, uploaded, downloads, uploader,
                    ));
                }
            }
            Ok(Response::Subtitle(page, subtitles))
        } else {
            let mut movies = Vec::new();
            if let Some(table) = document.select(&table_selector).next() {
                // skip 1 (table header)
                for line in table.select(&line_selector).skip(1) {
                    let id: u64 = match line.attr("id") {
                        Some(id) => id.strip_prefix("name").unwrap_or(id),
                        _ => continue,
                    }
                    .parse()
                    .unwrap_or_default();

                    let name = line
                        .text()
                        .take(2) // Omit links in movie name
                        .filter(|text| !text.contains("Watch online"))
                        .collect::<Vec<_>>()
                        .first()
                        .map(|value| value.replace("\n", "").replace("\t", "").to_string())
                        .unwrap_or_default();

                    movies.push(model::Movie::new(id, name));
                }
            }
            Ok(Response::Movie(movies))
        }
    }
}
