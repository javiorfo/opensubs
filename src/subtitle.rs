use scraper::{Html, Selector};

use crate::Result;

#[derive(Debug, Default)]
pub struct Subtitle {
    id: u64,
    movie: String,
    name: String,
    language: String,
    cd: String,
    uploaded: String,
    downloads: u32,
    uploader: Option<String>,
    download_link: String,
}

impl Subtitle {
    pub fn new(
        id: u64,
        movie: String,
        name: String,
        language: String,
        cd: String,
        uploaded: String,
        downloads: u32,
        uploader: Option<String>,
    ) -> Self {
        Self {
            id,
            movie,
            name,
            language,
            cd,
            uploaded,
            downloads,
            uploader,
            download_link: format!("https://dl.opensubtitles.org/en/download/sub/{}", id),
        }
    }
}

#[derive(Debug, Default)]
pub struct Movie {
    id: u64,
    name: String,
    subtitle_link: String,
}
// https://www.opensubtitles.org/en/search/sublanguageid-spa,eng,spl/idmovie-1196

#[derive(Debug)]
pub enum Response {
    Movie(Vec<Movie>),
    Subtitle(Vec<Subtitle>),
}

impl Response {
    pub(crate) fn create(url: &str, html: &str) -> Result<Self> {
        let document = Html::parse_document(html);

        let table_selector = Selector::parse("table#search_results")?;
        let line_selector = Selector::parse("tr")?;
        let column_selector = Selector::parse("td")?;

        if url.contains("imdbid") {
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
                        .map(|value| value.to_string())
                        .unwrap_or_default();

                    let name = movie_text
                        .get(2)
                        .map(|value| value.to_string())
                        .unwrap_or_default();

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

                    let cd = match data.next() {
                        Some(column) => column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string(),
                        None => "".to_string(),
                    };

                    let uploaded = match data.next() {
                        Some(column) => column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string(),
                        None => "".to_string(),
                    };

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

                    subtitles.push(Subtitle::new(
                        id, movie, name, language, cd, uploaded, downloads, uploader,
                    ));
                }
            }
            Ok(Response::Subtitle(subtitles))
        } else {
            Ok(Response::Movie(vec![]))
        }
    }
}
