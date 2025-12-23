//! JWT Claims for authentication.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::RUserRole;

/// JWT claims extracted from authentication token.
///
/// Used as extractor in web frameworks (axum/actix).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (subject).
    pub sub: Uuid,

    /// User role.
    pub role: RUserRole,

    /// Expiration time (Unix timestamp).
    pub exp: usize,

    /// Issued at (Unix timestamp).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iat: Option<usize>
}

impl Claims {
    /// Create new claims.
    #[must_use]
    pub fn new(sub: Uuid, role: RUserRole, exp: usize) -> Self {
        Self {
            sub,
            role,
            exp,
            iat: None
        }
    }

    /// Get user ID.
    #[must_use]
    pub fn user_id(&self) -> Uuid {
        self.sub
    }

    /// Check if claims are expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as usize)
            .unwrap_or(0);

        self.exp < now
    }
}
