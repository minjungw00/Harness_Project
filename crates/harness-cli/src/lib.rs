#![forbid(unsafe_code)]

//! Shared administrative CLI implementation pieces.
//!
//! The binary owns command-line parsing and process output. Library modules are
//! kept reusable so setup planning can be tested without invoking the binary.

pub mod registration;
pub mod setup;
