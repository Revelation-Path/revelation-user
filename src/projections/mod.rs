//! User projections for different contexts.
//!
//! Projections are read-only views of the User entity,
//! optimized for specific use cases.

mod auth;
mod public;

pub use auth::*;
pub use public::*;
