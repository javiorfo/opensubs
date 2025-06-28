#[derive(Debug)]
pub enum SearchBy<'a> {
    Url(&'a str),
    Movie(&'a str),
    MovieAndFilter(&'a str, Filter<'a>),
}

impl<'a> AsRef<SearchBy<'a>> for SearchBy<'a> {
    fn as_ref(&self) -> &SearchBy<'a> {
        self
    }
}

impl From<&SearchBy<'_>> for String {
    fn from(value: &SearchBy) -> Self {
        match value {
            SearchBy::Url(url) => url.to_string(),
            SearchBy::Movie(movie) => format!(
                "https://www.opensubtitles.org/en/search2?MovieName={}&id=8&action=search",
                movie.trim()
            ),
            SearchBy::MovieAndFilter(movie, filter) => format!(
                "https://www.opensubtitles.org/en/search2?MovieName={}&id=8&action=search{}",
                movie.trim(),
                filter.create()
            ),
        }
    }
}

impl<'a> SearchBy<'a> {
    pub(crate) fn filter(&self) -> Option<&Filter<'a>> {
        match self {
            SearchBy::MovieAndFilter(_, filter) => Some(filter),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Filters<'a>(Filter<'a>);

impl Default for Filters<'_> {
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
    pub fn year(mut self, year: u32) -> Self {
        self.0.year = year;
        self
    }

    pub fn languages(mut self, languages: &'a [Language]) -> Self {
        self.0.languages = languages;
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.0.page = page;
        self
    }

    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.0.order_by = order_by;
        self
    }

    pub fn build(self) -> Filter<'a> {
        self.0
    }
}

#[derive(Debug)]
pub struct Filter<'a> {
    year: u32,
    languages: &'a [Language],
    page: u32,
    order_by: OrderBy,
}

impl Filter<'_> {
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

    pub(crate) fn languages_to_str(&self) -> String {
        self.languages
            .iter()
            .map(|&lang| {
                let lang_str: &str = lang.into();
                lang_str
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    pub(crate) fn offset(&self) -> Option<String> {
        (self.page > 1).then_some(format!("/offset={}", (self.page - 1) * 40))
    }

    pub(crate) fn sort(&self) -> Option<&str> {
        self.order_by.sort()
    }
}

#[derive(Debug)]
pub enum OrderBy {
    Uploaded,
    Downloads,
    Rating,
}

impl OrderBy {
    pub(crate) fn sort(&self) -> Option<&str> {
        match self {
            Self::Uploaded => Some("/sort-5/asc-0"),
            Self::Downloads => Some("/sort-7/asc-0"),
            Self::Rating => Some("/sort-6/asc-0"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
