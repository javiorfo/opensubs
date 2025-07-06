//! # A library to search subtitles from opensubtitles.org
//!
//! This crate provides a high-level, ergonomic API for searching and retrieving subtitles and related metadata
//! from opensubtitles.org. It offers both asynchronous and blocking (synchronous) interfaces, with flexible
//! filtering and ordering options.
//!
//! ## Features
//!
//! - Search for subtitles using various criteria (language, filters, ordering, etc.).
//! - Retrieve detailed information about movies and subtitles.
//! - Both async and blocking APIs (enable via crate features).
//! - Strong error handling with [`Error`] and [`Result`] types.
//!
//! ## Usage
//!
//! Add this crate to your `Cargo.toml`:
//!
//! ```
//! [dependencies]
//! opensubs = "0.1.1"
//! ```
//!
//! #### Enable blocking feature if needed
//!
//! ```
//! [dependencies]
//! opensubs = { version = "0.1.1", features = ["blocking"] }
//! ```
//!
//! ### Async Example (default)
//!
//! ```
//! # #[cfg(feature = "async")]
//! use opensubs::{Filters, Language, OrderBy, SearchBy};
//!
//! #[tokio::main]
//! async fn main() -> opensubs::Result {
//!     // async search movie "holdovers", spanish subs, order by rating
//!     let results = opensubs::search(SearchBy::MovieAndFilter(
//!         "holdovers",
//!         Filters::default()
//!             .languages(&[Language::Spanish])
//!             .order_by(OrderBy::Rating)
//!             .build(),
//!     ))
//!     .await?;
//!
//!     println!("Subtitles {results:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Blocking Example (feature "blocking")
//!
//! ```
//! # #[cfg(feature = "blocking")]
//! use opensubs::{Filters, Language, OrderBy, Response, SearchBy};
//!
//! fn main() -> opensubs::Result {
//!     // blocking search movie "the godfather"
//!     // year 1972, french and german subs, order by rating
//!     let results = opensubs::blocking::search(SearchBy::MovieAndFilter(
//!         "the godfather",
//!         Filters::default()
//!             .year(1972)
//!             .languages(&[Language::French, Language::German])
//!             .order_by(OrderBy::Downloads)
//!             .build(),
//!     ))?;
//!
//!     match results {
//!         Response::Movie(movies) => {
//!             // If results is Movie type, get the subtitles_link property
//!             // and find subtitles for it
//!             if let Some(movie) = movies.first() {
//!                 let subs = opensubs::blocking::search(SearchBy::Url(&movie.subtitles_link))?;
//!                 println!("Subtitles {subs:#?}");
//!             }
//!         }
//!         // else print the subtitles
//!         _ => println!("Subtitles {results:#?}"),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Modules & Re-exports
//!
//! - [`client`] — Search options, filters, and search implementations.
//! - [`core`] — Core types, response parsing, and subtitle/movie models.
//! - [`Page`], [`Response`], [`Movie`], [`Subtitle`] — Main data structures for results.
//! - [`Filters`], [`Language`], [`OrderBy`], [`SearchBy`] — Search configuration types.
//!
//! ## Error Handling
//!
//! All fallible operations return [`Result<T>`](Result) with a custom [`Error`] enum that wraps
//! errors from underlying dependencies (e.g., `reqwest`, `scraper`).
//!
//! ## Feature Flags
//!
//! - `async` — Enables the asynchronous API (`search`).
//! - `blocking` — Enables the blocking (synchronous) API (`blocking::search`).
//!
//! ## License
//!
//! This is free software, published under the [MIT License](https://mit-license.org/).
//!
//! ## See Also
//!
//! - [`reqwest`] — HTTP client for requests.
//! - [`scraper`] — HTML parsing for subtitle extraction.

mod client;
mod core;

pub use client::{Filters, Language, OrderBy, SearchBy};
pub use core::{
    Page, Response,
    model::{Movie, Subtitle},
};

#[cfg(feature = "async")]
pub use client::default::search;

#[cfg(feature = "blocking")]
pub use client::blocking;

/// Error type for all fallible operations in this crate.
///
/// Wraps errors from underlying dependencies such as [`reqwest`] and [`scraper`].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    SelectorError(#[from] scraper::error::SelectorErrorKind<'static>),
}

/// A convenient alias for `Result` with the crate's [`Error`] type.
///
/// Defaults to `()` for the success type if not specified.
pub type Result<T = ()> = std::result::Result<T, Error>;
