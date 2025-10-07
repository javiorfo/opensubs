// Specifies the method and parameters for searching subtitles.
///
/// This enum allows you to search by a direct URL, by movie name, or by movie name with additional filters.
#[derive(Debug)]
pub enum SearchBy<'a> {
    /// Search using a direct URL.
    Url(&'a str),
    /// Search by movie name.
    Movie(&'a str),
    /// Search by movie name with additional filters.
    MovieAndFilter(&'a str, Filter<'a>),
}

impl<'a> AsRef<SearchBy<'a>> for SearchBy<'a> {
    fn as_ref(&self) -> &SearchBy<'a> {
        self
    }
}

impl From<&SearchBy<'_>> for String {
    /// Converts a `SearchBy` variant into a URL string for querying OpenSubtitles.
    fn from(value: &SearchBy) -> Self {
        let mut url = reqwest::Url::parse("https://www.opensubtitles.org/en/search2").unwrap();
        match value {
            SearchBy::Url(url) => url.to_string(),
            SearchBy::Movie(movie) => {
                url.query_pairs_mut().append_pair("MovieName", movie.trim());
                format!(
                    "https://www.opensubtitles.org/en/search2?MovieName={}&id=8&action=search",
                    url.as_str()
                )
            }
            SearchBy::MovieAndFilter(movie, filter) => {
                url.query_pairs_mut().append_pair("MovieName", movie.trim());
                format!("{}&id=8&action=search{}", url.as_str(), filter.create())
            }
        }
    }
}

impl<'a> SearchBy<'a> {
    /// Returns a reference to the filter if present (`MovieAndFilter` variant), otherwise `None`.
    pub(crate) fn filter(&self) -> Option<&Filter<'a>> {
        match self {
            SearchBy::MovieAndFilter(_, filter) => Some(filter),
            _ => None,
        }
    }
}

/// Builder for constructing a [`Filter`] with custom parameters.
///
/// # Example
/// ```
/// use opensubs::{Filters, Language, OrderBy};
///
/// let filter = Filters::default()
///     .year(2020)
///     .languages(&[Language::English])
///     .page(2)
///     .order_by(OrderBy::Downloads)
///     .build();
/// ```
#[derive(Debug)]
pub struct Filters<'a>(Filter<'a>);

impl Default for Filters<'_> {
    /// Creates a `Filters` builder with default parameters.
    fn default() -> Self {
        Self(Filter {
            year: 0,
            languages: &[],
            page: 1,
            order_by: OrderBy::Uploaded,
        })
    }
}

impl<'a> Filters<'a> {
    /// Sets the year filter.
    pub fn year(mut self, year: u32) -> Self {
        self.0.year = year;
        self
    }

    /// Sets the languages filter.
    pub fn languages(mut self, languages: &'a [Language]) -> Self {
        self.0.languages = languages;
        self
    }

    /// Sets the pagination page.
    pub fn page(mut self, page: u32) -> Self {
        self.0.page = page;
        self
    }

    /// Sets the sorting order.
    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.0.order_by = order_by;
        self
    }

    /// Builds and returns the configured [`Filter`].
    pub fn build(self) -> Filter<'a> {
        self.0
    }
}

/// Represents search filters for querying subtitles.
///
/// This struct is usually created via the [`Filters`] builder.
#[derive(Debug)]
pub struct Filter<'a> {
    /// Year to filter by (0 means no filter).
    year: u32,
    /// Languages to filter by.
    languages: &'a [Language],
    /// Page number for pagination (1-based).
    page: u32,
    /// Sorting order.
    order_by: OrderBy,
}

impl Filter<'_> {
    /// Creates a query string for the filter parameters.
    pub(crate) fn create(&self) -> String {
        let year = if self.year != 0 {
            self.year.to_string()
        } else {
            Default::default()
        };

        format!(
            "&SubLanguageID={}&MovieYearSign=1&MovieYear={}",
            self.languages_to_str(),
            year
        )
    }

    /// Returns a comma-separated string of language codes.
    pub(crate) fn languages_to_str(&self) -> String {
        self.languages
            .iter()
            .map(|lang| {
                let lang_str: &str = lang.clone().into();
                lang_str
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Returns the offset string for pagination if the page is greater than 1.
    pub(crate) fn offset(&self) -> Option<String> {
        (self.page > 1).then_some(format!("/offset={}", (self.page - 1) * 40))
    }

    /// Returns the sort string for the current `OrderBy` option.
    pub(crate) fn sort(&self) -> Option<&str> {
        self.order_by.sort()
    }
}

// Specifies the sorting order for search results.
#[derive(Debug, Clone)]
pub enum OrderBy {
    /// Sort by upload date (default).
    Uploaded,
    /// Sort by number of downloads.
    Downloads,
    /// Sort by rating.
    Rating,
}

impl OrderBy {
    /// Returns the corresponding sort string for the order.
    pub(crate) fn sort(&self) -> Option<&str> {
        match self {
            Self::Uploaded => Some("/sort-5/asc-0"),
            Self::Downloads => Some("/sort-7/asc-0"),
            Self::Rating => Some("/sort-6/asc-0"),
        }
    }
}

/// Represents all supported subtitle languages.
///
/// Each variant corresponds to a language supported by OpenSubtitles.
/// Use this enum to specify subtitle languages in search filters and queries.
///
/// # Example
/// ```
/// use opensubs::Language;
///
/// let lang = Language::English;
/// let code: &str = lang.into();
/// assert_eq!(code, "eng");
/// ```
#[derive(Debug, Clone)]
pub enum Language {
    Abkhazian,
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Aragonese,
    Armenian,
    Assamese,
    Asturian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Breton,
    Bulgarian,
    Burmese,
    Catalan,
    ChineseCantonese,
    ChineseSimplified,
    ChineseTraditional,
    ChineseBilingual,
    Croatian,
    Czech,
    Danish,
    Dari,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Extremaduran,
    Finnish,
    French,
    Gaelic,
    Galician,
    Georgian,
    German,
    Greek,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Igbo,
    Indonesian,
    Interlingua,
    Irish,
    Italian,
    Japanese,
    Kannada,
    Kazakh,
    Khmer,
    Korean,
    Kurdish,
    Kyrgyz,
    Latvian,
    Lithuanian,
    Luxembourgish,
    Macedonian,
    Malay,
    Malayalam,
    Manipuri,
    Marathi,
    Mongolian,
    Montenegrin,
    Navajo,
    Nepali,
    NorthernSami,
    Norwegian,
    Occitan,
    Odia,
    Persian,
    Polish,
    Portuguese,
    PortugueseBr,
    PortugueseMz,
    Pushto,
    Romanian,
    Russian,
    Santali,
    Serbian,
    Sindhi,
    Sinhalese,
    Slovak,
    Slovenian,
    Somali,
    SorbianLanguages,
    SouthAzerbaijani,
    Spanish,
    SpanishEU,
    SpanishLA,
    Swahili,
    Swedish,
    Syriac,
    Tagalog,
    Tamil,
    Tatar,
    Telugu,
    Tetum,
    Thai,
    TokiPona,
    Turkish,
    Turkmen,
    Ukrainian,
    Urdu,
    Uzbek,
    Vietnamese,
    Welsch,
}

/// Converts a [`Language`] variant into its OpenSubtitles language code as a `&str`.
///
/// # Example
/// ```
/// use opensubs::Language;
///
/// let code: &str = Language::PortugueseBr.into();
/// assert_eq!(code, "pob");
/// ```
impl From<Language> for &str {
    fn from(value: Language) -> Self {
        match value {
            Language::Abkhazian => "abk",
            Language::Afrikaans => "afr",
            Language::Albanian => "alb",
            Language::Amharic => "Amh",
            Language::Arabic => "ara",
            Language::Aragonese => "arg",
            Language::Armenian => "arm",
            Language::Assamese => "asm",
            Language::Asturian => "ast",
            Language::Azerbaijani => "aze",
            Language::Basque => "baq",
            Language::Belarusian => "bel",
            Language::Bengali => "ben",
            Language::Bosnian => "bos",
            Language::Breton => "bre",
            Language::Bulgarian => "bul",
            Language::Burmese => "bur",
            Language::Catalan => "cat",
            Language::ChineseCantonese => "zhc",
            Language::ChineseSimplified => "chi",
            Language::ChineseTraditional => "zht",
            Language::ChineseBilingual => "zhe",
            Language::Croatian => "hrv",
            Language::Czech => "cze",
            Language::Danish => "dan",
            Language::Dari => "prs",
            Language::Dutch => "dut",
            Language::English => "eng",
            Language::Esperanto => "epo",
            Language::Estonian => "est",
            Language::Extremaduran => "ext",
            Language::Finnish => "fin",
            Language::French => "fre",
            Language::Gaelic => "gla",
            Language::Galician => "glb",
            Language::Georgian => "geo",
            Language::German => "ger",
            Language::Greek => "ell",
            Language::Hebrew => "heb",
            Language::Hindi => "hin",
            Language::Hungarian => "hun",
            Language::Icelandic => "ice",
            Language::Igbo => "ibo",
            Language::Indonesian => "ind",
            Language::Interlingua => "ina",
            Language::Irish => "gle",
            Language::Italian => "ita",
            Language::Japanese => "jpn",
            Language::Kannada => "kan",
            Language::Kazakh => "kaz",
            Language::Khmer => "khm",
            Language::Korean => "kor",
            Language::Kurdish => "kur",
            Language::Kyrgyz => "kir",
            Language::Latvian => "lav",
            Language::Lithuanian => "lit",
            Language::Luxembourgish => "ltz",
            Language::Macedonian => "mac",
            Language::Malay => "may",
            Language::Malayalam => "mal",
            Language::Manipuri => "mni",
            Language::Marathi => "mar",
            Language::Mongolian => "mon",
            Language::Montenegrin => "mne",
            Language::Navajo => "nav",
            Language::Nepali => "nep",
            Language::NorthernSami => "sme",
            Language::Norwegian => "nor",
            Language::Occitan => "oci",
            Language::Odia => "ori",
            Language::Persian => "per",
            Language::Polish => "pol",
            Language::Portuguese => "por",
            Language::PortugueseBr => "pob",
            Language::PortugueseMz => "pom",
            Language::Pushto => "pus",
            Language::Romanian => "rum",
            Language::Russian => "rus",
            Language::Santali => "sat",
            Language::Serbian => "scc",
            Language::Sindhi => "snd",
            Language::Sinhalese => "sin",
            Language::Slovak => "slo",
            Language::Slovenian => "slv",
            Language::Somali => "som",
            Language::SorbianLanguages => "wen",
            Language::SouthAzerbaijani => "azb",
            Language::Spanish => "spa",
            Language::SpanishEU => "spn",
            Language::SpanishLA => "spl",
            Language::Swahili => "swa",
            Language::Swedish => "swe",
            Language::Syriac => "syr",
            Language::Tagalog => "tgl",
            Language::Tamil => "tam",
            Language::Tatar => "tat",
            Language::Telugu => "tel",
            Language::Tetum => "tet",
            Language::Thai => "tha",
            Language::TokiPona => "tok",
            Language::Turkish => "tur",
            Language::Turkmen => "tuk",
            Language::Ukrainian => "ukr",
            Language::Urdu => "urd",
            Language::Uzbek => "uzb",
            Language::Vietnamese => "vie",
            Language::Welsch => "wel",
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "abkhazian" => Ok(Language::Abkhazian),
            "afrikaans" => Ok(Language::Afrikaans),
            "albanian" => Ok(Language::Albanian),
            "amharic" => Ok(Language::Amharic),
            "arabic" => Ok(Language::Arabic),
            "aragonese" => Ok(Language::Aragonese),
            "armenian" => Ok(Language::Armenian),
            "assamese" => Ok(Language::Assamese),
            "asturian" => Ok(Language::Asturian),
            "azerbaijani" => Ok(Language::Azerbaijani),
            "basque" => Ok(Language::Basque),
            "belarusian" => Ok(Language::Belarusian),
            "bengali" => Ok(Language::Bengali),
            "bosnian" => Ok(Language::Bosnian),
            "breton" => Ok(Language::Breton),
            "bulgarian" => Ok(Language::Bulgarian),
            "burmese" => Ok(Language::Burmese),
            "catalan" => Ok(Language::Catalan),
            "chinese cantonese" => Ok(Language::ChineseCantonese),
            "chinese simplified" => Ok(Language::ChineseSimplified),
            "chinese traditional" => Ok(Language::ChineseTraditional),
            "chinese bilingual" => Ok(Language::ChineseBilingual),
            "croatian" => Ok(Language::Croatian),
            "czech" => Ok(Language::Czech),
            "danish" => Ok(Language::Danish),
            "dari" => Ok(Language::Dari),
            "dutch" => Ok(Language::Dutch),
            "english" => Ok(Language::English),
            "esperanto" => Ok(Language::Esperanto),
            "estonian" => Ok(Language::Estonian),
            "extremaduran" => Ok(Language::Extremaduran),
            "finnish" => Ok(Language::Finnish),
            "french" => Ok(Language::French),
            "gaelic" => Ok(Language::Gaelic),
            "galician" => Ok(Language::Galician),
            "georgian" => Ok(Language::Georgian),
            "german" => Ok(Language::German),
            "greek" => Ok(Language::Greek),
            "hebrew" => Ok(Language::Hebrew),
            "hindi" => Ok(Language::Hindi),
            "hungarian" => Ok(Language::Hungarian),
            "icelandic" => Ok(Language::Icelandic),
            "igbo" => Ok(Language::Igbo),
            "indonesian" => Ok(Language::Indonesian),
            "interlingua" => Ok(Language::Interlingua),
            "irish" => Ok(Language::Irish),
            "italian" => Ok(Language::Italian),
            "japanese" => Ok(Language::Japanese),
            "kannada" => Ok(Language::Kannada),
            "kazakh" => Ok(Language::Kazakh),
            "khmer" => Ok(Language::Khmer),
            "korean" => Ok(Language::Korean),
            "kurdish" => Ok(Language::Kurdish),
            "kyrgyz" => Ok(Language::Kyrgyz),
            "latvian" => Ok(Language::Latvian),
            "lithuanian" => Ok(Language::Lithuanian),
            "luxembourgish" => Ok(Language::Luxembourgish),
            "macedonian" => Ok(Language::Macedonian),
            "malay" => Ok(Language::Malay),
            "malayalam" => Ok(Language::Malayalam),
            "manipuri" => Ok(Language::Manipuri),
            "marathi" => Ok(Language::Marathi),
            "mongolian" => Ok(Language::Mongolian),
            "montenegrin" => Ok(Language::Montenegrin),
            "navajo" => Ok(Language::Navajo),
            "nepali" => Ok(Language::Nepali),
            "northern sami" => Ok(Language::NorthernSami),
            "norwegian" => Ok(Language::Norwegian),
            "occitan" => Ok(Language::Occitan),
            "odia" => Ok(Language::Odia),
            "persian" => Ok(Language::Persian),
            "polish" => Ok(Language::Polish),
            "portuguese" => Ok(Language::Portuguese),
            "portuguese br" => Ok(Language::PortugueseBr),
            "portuguese mz" => Ok(Language::PortugueseMz),
            "pushto" => Ok(Language::Pushto),
            "romanian" => Ok(Language::Romanian),
            "russian" => Ok(Language::Russian),
            "santali" => Ok(Language::Santali),
            "serbian" => Ok(Language::Serbian),
            "sindhi" => Ok(Language::Sindhi),
            "sinhalese" => Ok(Language::Sinhalese),
            "slovak" => Ok(Language::Slovak),
            "slovenian" => Ok(Language::Slovenian),
            "somali" => Ok(Language::Somali),
            "sorbian languages" => Ok(Language::SorbianLanguages),
            "south azerbaijani" => Ok(Language::SouthAzerbaijani),
            "spanish" => Ok(Language::Spanish),
            "spanish eu" => Ok(Language::SpanishEU),
            "spanish la" => Ok(Language::SpanishLA),
            "swahili" => Ok(Language::Swahili),
            "swedish" => Ok(Language::Swedish),
            "syriac" => Ok(Language::Syriac),
            "tagalog" => Ok(Language::Tagalog),
            "tamil" => Ok(Language::Tamil),
            "tatar" => Ok(Language::Tatar),
            "telugu" => Ok(Language::Telugu),
            "tetum" => Ok(Language::Tetum),
            "thai" => Ok(Language::Thai),
            "toki pona" => Ok(Language::TokiPona),
            "turkish" => Ok(Language::Turkish),
            "turkmen" => Ok(Language::Turkmen),
            "ukrainian" => Ok(Language::Ukrainian),
            "urdu" => Ok(Language::Urdu),
            "uzbek" => Ok(Language::Uzbek),
            "vietnamese" => Ok(Language::Vietnamese),
            "welsch" => Ok(Language::Welsch),
            _ => Err(format!("Invalid value for Language: {value}")),
        }
    }
}

/// Implements the `Display` trait for the `Language` enum.
///
/// This implementation allows `Language` variants to be easily formatted into
/// user-facing strings using the `{}` format specifier. It uses the `Debug`
/// implementation to get a simple string representation of the enum variant
/// name (e.g., `English`, `Spanish`).
impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use a match statement to convert each enum variant to a string
        write!(f, "{:?}", self)
    }
}
