// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use exitcode::ExitCode;
use structopt::{
    clap::{self, crate_version, ErrorKind},
    StructOpt,
};

use super::log;

const RIFTD: &str = "riftd";

/// Overall `riftd` binary configuration.
#[derive(Debug, Clone, StructOpt)]
#[structopt(
    global_settings = &[clap::AppSettings::DeriveDisplayOrder],
    author = "Christian Saide <me@csaide.dev>",
    about = "Run an instance of riftd.",
    version = crate_version!()
)]
struct Config {
    #[structopt(flatten)]
    log_config: log::Config,
}

/// The primary entrypoint function for the `riftd` binary.
pub fn run() -> ExitCode {
    let cfg = match Config::from_args_safe() {
        Ok(cfg) => cfg,
        Err(err)
            if err.kind == ErrorKind::HelpDisplayed || err.kind == ErrorKind::VersionDisplayed =>
        {
            println!("{}", err.message);
            return exitcode::USAGE;
        }
        Err(err) => {
            println!("{}", err.message);
            return exitcode::CONFIG;
        }
    };

    let logger = log::new(&cfg.log_config, RIFTD);
    info!(logger, "Hello World!");

    exitcode::OK
}
