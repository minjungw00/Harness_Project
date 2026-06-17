#![forbid(unsafe_code)]

//! Shared implementation-test helpers.
//!
//! Helpers in this crate should use disposable locations, such as `/tmp`, for
//! future runtime homes and fixture output.

use std::path::PathBuf;

use harness_types::TypeBoundary;

pub mod fixtures {
    /// Placement marker for future shared fixtures.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct FixtureBoundary;
}

pub mod golden {
    /// Placement marker for future golden-output helpers.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct GoldenBoundary;
}

/// Returns a candidate disposable runtime-home path without creating it.
pub fn disposable_runtime_home(name: &str) -> PathBuf {
    std::env::temp_dir().join("harness-test-runtime").join(name)
}

/// Identifies the shared type boundary used by test helpers.
pub const fn shared_type_boundary() -> TypeBoundary {
    TypeBoundary::Domain
}

#[cfg(test)]
mod tests {
    use super::{disposable_runtime_home, shared_type_boundary};
    use harness_types::TypeBoundary;

    #[test]
    fn disposable_runtime_home_stays_under_system_temp() {
        let path = disposable_runtime_home("workspace-skeleton");
        assert!(path.is_absolute());
        assert!(path.ends_with("harness-test-runtime/workspace-skeleton"));
    }

    #[test]
    fn test_support_uses_domain_type_boundary() {
        assert_eq!(shared_type_boundary(), TypeBoundary::Domain);
    }
}
