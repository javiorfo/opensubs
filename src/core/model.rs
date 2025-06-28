use crate::Filter;

/// Represents a subtitle entry with metadata and download information.
#[derive(Debug, Default)]
pub struct Subtitle {
    /// Unique identifier for the subtitle.
    pub id: u64,
    /// Movie title associated with the subtitle.
    pub movie: String,
    /// Optional name or description of the subtitle.
    pub name: Option<String>,
    /// Language code of the subtitle (e.g., "eng" for English).
    pub language: String,
    /// CD or disc information (e.g., "CD1", "CD2").
    pub cd: String,
    /// Upload date or timestamp.
    pub uploaded: String,
    /// Number of times the subtitle has been downloaded.
    pub downloads: u32,
    /// User rating for the subtitle.
    pub rating: f32,
    /// Optional uploader's username.
    pub uploader: Option<String>,
    /// Direct download link for the subtitle file.
    pub download_link: String,
}

#[allow(clippy::too_many_arguments)]
impl Subtitle {
    /// Creates a new `Subtitle` instance with the provided metadata.
    ///
    /// The `download_link` is automatically generated based on the subtitle ID.
    ///
    /// # Arguments
    /// * `id` - Subtitle ID.
    /// * `movie` - Movie title.
    /// * `name` - Optional subtitle name.
    /// * `language` - Language code.
    /// * `cd` - CD/disc information.
    /// * `uploaded` - Upload date.
    /// * `downloads` - Number of downloads.
    /// * `rating` - Subtitle rating.
    /// * `uploader` - Optional uploader's username.
    pub(crate) fn new(
        id: u64,
        movie: String,
        name: Option<String>,
        language: String,
        cd: String,
        uploaded: String,
        downloads: u32,
        rating: f32,
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
            rating,
            uploader,
            download_link: format!("https://dl.opensubtitles.org/en/download/sub/{id}"),
        }
    }

    /// Checks if a given URL refers to a subtitle resource.
    ///
    /// Returns `true` if the URL contains `"imdbid"` or `"idmovie"`.
    pub(crate) fn is_subtitle(url: &str) -> bool {
        url.contains("imdbid") || url.contains("idmovie")
    }

    /// Modifies the given URL by appending filter parameters if it is a subtitle URL.
    ///
    /// # Arguments
    /// * `url` - The URL to process.
    /// * `filter` - Optional reference to a `Filter` for offset and sort parameters.
    pub(crate) fn process_url(url: &mut String, filter: Option<&Filter>) {
        if Self::is_subtitle(url) {
            let offset = filter.and_then(|f| f.offset()).unwrap_or_default();
            let sort = filter.and_then(|f| f.sort()).unwrap_or_default();

            url.push_str(&offset);
            url.push_str(sort);
        }
    }
}

/// Represents a movie with an associated subtitles search link.
#[derive(Debug, Default)]
pub struct Movie {
    /// Unique identifier for the movie.
    pub id: u64,
    /// Movie title.
    pub name: String,
    /// URL to search for subtitles for this movie.
    pub subtitles_link: String,
}

impl Movie {
    /// Creates a new `Movie` instance with a generated subtitles search link.
    ///
    /// # Arguments
    /// * `id` - Movie ID.
    /// * `name` - Movie title.
    /// * `languages` - Language filter for subtitles (e.g., "eng").
    /// * `offset` - Pagination offset.
    /// * `sort` - Sort parameter.
    pub(crate) fn new(id: u64, name: String, languages: &str, offset: &str, sort: &str) -> Self {
        Self {
            id,
            name,
            subtitles_link: format!(
                "https://www.opensubtitles.org/en/search/sublanguageid-{languages}/idmovie-{id}{offset}{sort}"
            ),
        }
    }
}
