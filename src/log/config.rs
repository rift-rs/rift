// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later-only

use structopt::StructOpt;

use super::level::Level;

#[derive(Debug, Clone, StructOpt)]
/// Rift logging configuration.
pub struct Config {
    #[structopt(
        long = "log-level",
        short = "l",
        env = "RIFT_LOG_LEVEL",
        help = "The logging level to use.",
        long_help = "Selects the maximum logging level to log for all application logs.",
        default_value = "info",
        possible_values = &["critical", "error", "warn", "info", "debug"],
        takes_value = true
    )]
    /// Define the logging level to use.
    pub level: Level,

    #[structopt(
        long = "log-json",
        short = "j",
        env = "RIFT_LOG_JSON",
        help = "Whether or not to log in JSON format.",
        long_help = "Sets whether or not to log in JSON format.",
        takes_value = false
    )]
    /// Define whether or not to log in json format.
    pub json: bool,
}
