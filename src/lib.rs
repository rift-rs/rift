// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0

#![warn(missing_docs)]

//! The `librift` library encapsulates the various modules required for the Rift ecosystem.

#[macro_use]
extern crate slog;

/// General logger implementation based ontop of slog.
pub mod log;
/// The entrypoint, configuration, and logic for the `riftd` binary.
pub mod riftd;
