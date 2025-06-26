mod client;
mod core;

pub use client::{Filter, Filters, Language, SearchBy};

#[cfg(feature = "async")]
pub use client::default;

#[cfg(feature = "blocking")]
pub use client::blocking;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    SelectorError(#[from] scraper::error::SelectorErrorKind<'static>),
}

pub type Result<T = ()> = std::result::Result<T, Error>;
