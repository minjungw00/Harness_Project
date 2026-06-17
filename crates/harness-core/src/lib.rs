#![forbid(unsafe_code)]

//! Core-facing services for future owner-defined Harness behavior.
//!
//! Public Harness method behavior is intentionally unimplemented in this
//! skeleton. Adapters may depend on this crate; this crate does not depend on
//! adapter crates.

use harness_store::{artifacts::ArtifactStoreBoundary, sqlite::SqliteStoreBoundary};
use harness_types::TypeBoundary;

/// Minimal Core service marker for validating crate boundaries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CoreBoundary {
    store: SqliteStoreBoundary,
    artifacts: ArtifactStoreBoundary,
}

impl CoreBoundary {
    /// Creates an inert Core boundary marker.
    pub const fn new() -> Self {
        Self {
            store: SqliteStoreBoundary,
            artifacts: ArtifactStoreBoundary,
        }
    }

    /// Identifies the shared type boundary Core will use for future APIs.
    pub const fn api_type_boundary(self) -> TypeBoundary {
        let _ = self.store;
        let _ = self.artifacts;
        TypeBoundary::Api
    }
}

#[cfg(test)]
mod tests {
    use super::CoreBoundary;
    use harness_types::TypeBoundary;

    #[test]
    fn core_boundary_points_to_api_types() {
        assert_eq!(CoreBoundary::new().api_type_boundary(), TypeBoundary::Api);
    }
}
