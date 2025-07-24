//! Gith - A friendly Git wrapper for marking human-generated content
//!
//! This library provides functionality to wrap Git operations and add
//! human-generation flags to commits and track human-generated content.

pub mod cli;
pub mod error;
pub mod git;
pub mod license;
pub mod manifest;

pub use error::{GithError, Result};
