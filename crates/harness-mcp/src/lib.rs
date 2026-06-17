#![forbid(unsafe_code)]

//! MCP adapter boundary for future transport-to-Core mapping.
//!
//! This crate does not implement MCP tool protocol behavior yet.

use harness_core::CoreBoundary;

/// Minimal MCP adapter marker for validating dependency direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McpAdapterBoundary {
    core: CoreBoundary,
}

impl McpAdapterBoundary {
    /// Creates an inert MCP adapter boundary marker.
    pub const fn new(core: CoreBoundary) -> Self {
        Self { core }
    }

    /// Returns the placeholder adapter label.
    pub const fn label(self) -> &'static str {
        let _ = self.core;
        "mcp-adapter"
    }
}

#[cfg(test)]
mod tests {
    use super::McpAdapterBoundary;
    use harness_core::CoreBoundary;

    #[test]
    fn mcp_boundary_wraps_core_boundary() {
        assert_eq!(
            McpAdapterBoundary::new(CoreBoundary::new()).label(),
            "mcp-adapter"
        );
    }
}
