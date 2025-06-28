//! # Subtitle Search Crate
//!
//! This crate provides a high-level, ergonomic API for searching and retrieving subtitles and related metadata
//! from supported sources. It offers both asynchronous and blocking (synchronous) interfaces, with flexible
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
//! opensubs = "0.1.0"
//! ```
//!
//! ### Async Example
//!
//! ```
//! # #[cfg(feature = "async")]
//! use opensubs::{SearchBy, search};
//!
//! # #[cfg(feature = "async")]
//! #[tokio::main]
//! async fn main() -> opensubs::Result<()> {
//!     let search_by = SearchBy::new("movie name");
//!     let response = search(search_by).await?;
//!     println!("{:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! ### Blocking Example
//!
//! ```
//! # #[cfg(feature = "blocking")]
//! use opensubs::{SearchBy, blocking};
//!
//! # #[cfg(feature = "blocking")]
//! fn main() -> opensubs::Result<()> {
//!     let search_by = SearchBy::new("movie name");
//!     let response = blocking::search(search_by)?;
//!     println!("{:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! ## Modules & Re-exports
//!
//! - [`client`] — Search options, filters, and search implementations.
//! - [`core`] — Core types, response parsing, and subtitle/movie models.
//! - [`Page`], [`Response`], [`Movie`], [`Subtitle`] — Main data structures for results.
//! - [`Filter`], [`Filters`], [`Language`], [`OrderBy`], [`SearchBy`] — Search configuration types.
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
//! Licensed under your preferred license.
//!
//! ## See Also
//!
//! - [`reqwest`] — HTTP client for requests.
//! - [`scraper`] — HTML parsing for subtitle extraction.

mod client;
mod core;

pub use client::{Filter, Filters, Language, OrderBy, SearchBy};
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

/// Convenient result type for this crate.
pub type Result<T = ()> = std::result::Result<T, Error>;
