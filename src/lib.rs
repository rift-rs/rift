// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

#![warn(missing_docs)]

//! The `librift` library encapsulates the various modules required for the Rift ecosystem.

#[macro_use]
extern crate slog;

/// General logger implementation based on the slog ecosystem.
pub mod log;
/// General metrics collection/management based on the prometheus ecosystem.
pub mod metrics;
/// The entrypoint, configuration, and logic for the `riftd` binary.
pub mod riftd;
