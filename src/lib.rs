#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//! #### Usage
//!
//! ```rust,no_run
//! use axiston::{Client, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Client::default();
//!     client.health().await?;
//!     Ok(())
//! }
//! ```

pub use builder::Builder;
pub use client::Client;
pub(crate) use config::Config;

mod builder;
mod client;
mod config;
mod services;

pub mod types {
    //! Request and response types.

    /// List specifying general categories of [`ErrorResponse`]s.
    #[derive(Debug, Copy, Clone)]
    pub enum ErrorKind {
        /// Error name is not a part of the implemented `API` spec.
        Unrecognized,
    }

    /// Errors that may occur when processing an API request.
    #[derive(Debug, thiserror::Error, serde::Deserialize)]
    #[error("'{name}': {message}")]
    pub struct ErrorResponse {
        pub name: String,
        pub message: String,

        #[serde(skip)]
        status: u16,
    }

    impl ErrorResponse {
        /// Returns the [`ErrorKind`].
        pub fn kind(&self) -> ErrorKind {
            match self.name.as_str() {
                _ => ErrorKind::Unrecognized,
            }
        }
    }
}

/// Error type for a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur when processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur when processing an API request.
    #[error("api error: {0}")]
    Api(#[from] types::ErrorResponse),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
