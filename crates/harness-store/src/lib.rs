#![forbid(unsafe_code)]

//! Storage boundary for SQLite records, artifact plumbing, and migrations.
//!
//! This crate intentionally contains no DDL, migrations, or storage effects yet.

use harness_types::TypeBoundary;

pub mod artifacts {
    /// Placement marker for future artifact-store plumbing.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ArtifactStoreBoundary;
}

pub mod migrations {
    /// Placement marker for future storage migration wiring.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MigrationBoundary;
}

pub mod sqlite {
    /// Placement marker for future SQLite-backed store code.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct SqliteStoreBoundary;
}

/// Identifies the shared type boundary this crate depends on.
pub const fn shared_type_boundary() -> TypeBoundary {
    TypeBoundary::Domain
}

#[cfg(test)]
mod tests {
    use super::shared_type_boundary;
    use harness_types::TypeBoundary;

    #[test]
    fn store_depends_on_domain_types_boundary() {
        assert_eq!(shared_type_boundary(), TypeBoundary::Domain);
    }
}
