#![forbid(unsafe_code)]

//! Shared Rust type boundary for Harness API and domain values.
//!
//! This crate is scaffolding only. Owner-defined schemas and value meanings
//! remain in the maintained Reference documents until implemented.

/// High-level placement marker for future shared type groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeBoundary {
    /// API-facing Rust types will live behind this boundary.
    Api,
    /// Core/domain Rust types will live behind this boundary.
    Domain,
}

impl TypeBoundary {
    /// Returns a stable implementation-facing label for the boundary marker.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Domain => "domain",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypeBoundary;

    #[test]
    fn boundary_labels_are_stable() {
        assert_eq!(TypeBoundary::Api.label(), "api");
        assert_eq!(TypeBoundary::Domain.label(), "domain");
    }
}
