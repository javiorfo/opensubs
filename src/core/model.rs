#[derive(Debug, Default)]
pub struct Subtitle {
    pub id: u64,
    pub movie: String,
    pub name: Option<String>,
    pub language: String,
    pub cd: String,
    pub uploaded: String,
    pub downloads: u32,
    pub rating: f32,
    pub uploader: Option<String>,
    pub download_link: String,
}

#[allow(clippy::too_many_arguments)]
impl Subtitle {
    pub fn new(
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
            download_link: format!("https://dl.opensubtitles.org/en/download/sub/{}", id),
        }
    }
}

#[derive(Debug, Default)]
pub struct Movie {
    pub id: u64,
    pub name: String,
    pub subtitles_link: String,
}

impl Movie {
    pub fn new(id: u64, name: String, languages: &str, offset: &str) -> Self {
        // TODO sublanguageid by parameter
        Self {
            id,
            name,
            subtitles_link: format!(
                "https://www.opensubtitles.org/en/search/sublanguageid-{}/idmovie-{}{}",
                languages, id, offset
            ),
        }
    }
}
