//! Framework-specific extractors for authentication.
//!
//! Enable with `axum` or `actix` feature (mutually exclusive).

#[cfg(all(feature = "axum", feature = "actix"))]
compile_error!("Features `axum` and `actix` are mutually exclusive");

#[cfg(feature = "axum")]
mod axum_extract;
#[cfg(feature = "axum")]
pub use axum_extract::*;

#[cfg(feature = "actix")]
mod actix_extract;
#[cfg(feature = "actix")]
pub use actix_extract::*;
