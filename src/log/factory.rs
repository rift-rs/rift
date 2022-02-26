// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0

use slog::Drain;
use structopt::clap::crate_version;

use super::{config, filter};

/// Return a newly constructed slog::Logger based on the supplied configuration.
/// This also injects the application name and version as base key/value pairs for the
/// returned root logger.
///
/// # Example
/// ```
/// use slog::info;
///
/// let logger = librift::log::new(
///     &librift::log::Config {
///         level: librift::log::Level::Info,
///         json: true,
///     },
///     "example",
/// );
///
/// info!(logger, "Hello world!"; "woot" => "woot");
/// ```
pub fn new(cfg: &config::Config, bin: &'static str) -> slog::Logger {
    let drain: Box<dyn Drain<Ok = (), Err = slog::Never> + Send> = if cfg.json {
        Box::new(
            slog_json::Json::new(std::io::stdout())
                .add_default_keys()
                .build()
                .fuse(),
        )
    } else {
        let decorator = slog_term::TermDecorator::new().build();
        Box::new(
            slog_term::FullFormat::new(decorator)
                .use_utc_timestamp()
                .build()
                .fuse(),
        )
    };

    let drain = filter::LevelFilter {
        drain,
        level: cfg.level.to_slog(),
    }
    .fuse();

    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("binary" => bin, "version" => crate_version!()))
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_json_logger() {
        let cfg = config::Config {
            json: true,
            level: crate::log::Level::Info,
        };
        let logger = new(&cfg, "testing");
        info!(logger, "Created new json logger!");
    }

    #[test]
    fn test_plain_logger() {
        let cfg = config::Config {
            json: false,
            level: crate::log::Level::Info,
        };
        let logger = new(&cfg, "testing");
        info!(logger, "Created new plain logger!");
    }
}
