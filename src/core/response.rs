use crate::{client::Filter, core::model::Subtitle};

use super::model;
use regex::Regex;
use scraper::{Html, Selector};

/// Represents pagination information for search results.
///
/// The `Page` struct holds the range (`from` to `to`) and the total number of items.
#[derive(Debug, Default)]
pub struct Page {
    /// The starting index of the current page.
    pub from: u32,
    /// The ending index of the current page.
    pub to: u32,
    /// The total number of items available.
    pub total: u32,
}

impl From<Option<String>> for Page {
    /// Parses a string (extracted from HTML) to create a `Page`.
    ///
    /// The string should contain at least three numbers: `from`, `to`, and `total`.
    /// If parsing fails, returns the default page (all fields set to zero).
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

/// Represents a parsed response from a search page.
///
/// The response can either be a list of movies or a list of subtitles with pagination.
#[derive(Debug)]
pub enum Response {
    /// A list of movies found in the search results.
    Movie(Vec<model::Movie>),
    /// A paginated list of subtitles found in the search results.
    Subtitle(Page, Vec<model::Subtitle>),
}

impl Response {
    /// Parses an HTML search result page and constructs a `Response`.
    ///
    /// Depending on the URL, this function will parse either a list of movies or a list of subtitles.
    ///
    /// # Arguments
    /// * `url` - The URL of the search page.
    /// * `html` - The HTML content of the page.
    /// * `filter` - Optional filter to apply for language, offset, and sort.
    ///
    /// # Returns
    /// * `Response::Movie` if the page contains a list of movies.
    /// * `Response::Subtitle` if the page contains a list of subtitles, along with pagination info.
    ///
    /// # Errors
    /// Returns an error if HTML parsing or selector creation fails.
    pub(crate) fn create(url: &str, html: &str, filter: Option<&Filter>) -> crate::Result<Self> {
        let document = Html::parse_document(html);

        let table_selector = Selector::parse("table#search_results")?;
        let line_selector = Selector::parse("tr")?;
        let column_selector = Selector::parse("td")?;

        if Subtitle::is_subtitle(url) {
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

                    let rating: f32 = data
                        .next()
                        .and_then(|column| column.select(&Selector::parse("span").unwrap()).next())
                        .and_then(|column| column.text().next())
                        .and_then(|text| text.parse::<f32>().ok())
                        .unwrap_or_default();

                    let uploader = data.nth(2).and_then(|column| {
                        let name = column
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        if name.is_empty() { None } else { Some(name) }
                    });

                    subtitles.push(model::Subtitle::new(
                        id, movie, name, language, cd, uploaded, downloads, rating, uploader,
                    ));
                }
            }
            Ok(Response::Subtitle(page, subtitles))
        } else {
            let mut movies = Vec::new();
            if let Some(table) = document.select(&table_selector).next() {
                let languages = filter
                    .map(|f| f.languages_to_str())
                    .unwrap_or("all".to_string());

                let offset = filter.and_then(|f| f.offset()).unwrap_or_default();
                let sort = filter.and_then(|f| f.sort()).unwrap_or_default();

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

                    movies.push(model::Movie::new(id, name, &languages, &offset, sort));
                }
            }
            Ok(Response::Movie(movies))
        }
    }
}
