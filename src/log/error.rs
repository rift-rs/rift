// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later-only

use std::result;

use thiserror::Error;

/// Custom Result wrapper to simplify usage.
pub type Result<T> = result::Result<T, Error>;

/// Represents logging errors based on user configuration or OS
/// errors while attempting to configure log handlers.
#[derive(Error, Debug)]
pub enum Error {
    /// Handles errors for undefined or invalid log level conversions.
    #[error("invalid level specified: {level}")]
    InvalidLevel {
        /// level represents the level that was configued but unimplemented.
        level: String,
    },
}

impl From<&str> for Error {
    fn from(i: &str) -> Self {
        let level = i.to_owned();
        Self::InvalidLevel { level }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let str = "string";
        let error = Error::from(str);
        assert!(matches!(error, Error::InvalidLevel { .. }));
        assert_eq!("invalid level specified: string", format!("{}", error));
    }
}
