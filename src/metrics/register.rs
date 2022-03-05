// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use prometheus::{
    register_counter, register_counter_vec, register_gauge, register_gauge_vec, register_histogram,
    register_histogram_vec, register_int_counter, register_int_counter_vec, register_int_gauge,
    register_int_gauge_vec,
};
use structopt::clap::crate_version;

use crate::metrics::opt::to_histogram_opts;

use super::{opt::to_common_opts, Error, Opt, Result};

/// Register a new [f64] based counter using the default registry.
pub fn register_counter(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::Counter> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    register_counter!(opts).map_err(|e| Error::from(name, e))
}

/// Register a new [f64] based counter vec based on supplied [Opt::Labels] or [Opt::Label] user options.
pub fn register_counter_vec(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::CounterVec> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    let labels = opts.variable_labels.clone();
    let labels = labels.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    register_counter_vec!(opts, labels.as_ref()).map_err(|e| Error::from(name, e))
}

/// Register a new [u64] based integer counter using the default registry.
pub fn register_int_counter(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::IntCounter> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    register_int_counter!(opts).map_err(|e| Error::from(name, e))
}

/// Register a new [u64] based integer counter vec based on supplied [Opt::Labels] or [Opt::Label] user options.
pub fn register_int_counter_vec(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::IntCounterVec> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    let labels = opts.variable_labels.clone();
    let labels = labels.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    register_int_counter_vec!(opts, labels.as_ref()).map_err(|e| Error::from(name, e))
}

/// Register a new [f64] based gauge using the default registry.
pub fn register_gauge(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::Gauge> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    register_gauge!(opts).map_err(|e| Error::from(name, e))
}

/// Register a new [f64] based guage vec based on the supplied [Opt::Labels] or [Opt::Label] user options.
pub fn register_gauge_vec(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::GaugeVec> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    let labels = opts.variable_labels.clone();
    let labels = labels.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    register_gauge_vec!(opts, labels.as_ref()).map_err(|e| Error::from(name, e))
}

/// Register a new [u64] based gauge using the default registry.
pub fn register_int_gauge(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::IntGauge> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    register_int_gauge!(opts).map_err(|e| Error::from(name, e))
}

/// Register a new [u64] based guage vec based on the supplied [Opt::Labels] or [Opt::Label] user options.
pub fn register_int_gauge_vec(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::IntGaugeVec> {
    let name = name.into();
    let mut opts = to_common_opts(name.clone(), help, user_opts);
    opts.const_labels
        .insert(String::from("version"), crate_version!().to_string());

    let labels = opts.variable_labels.clone();
    let labels = labels.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    register_int_gauge_vec!(opts, labels.as_ref()).map_err(|e| Error::from(name, e))
}

/// Register a new [f64] based histogram.
pub fn register_histogram(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::Histogram> {
    let name = name.into();
    let mut opts = to_histogram_opts(name.clone(), help, user_opts);
    opts.common_opts
        .const_labels
        .insert(String::from("version"), crate_version!().to_string());

    register_histogram!(opts).map_err(|e| Error::from(name, e))
}

/// Register a new [f64] based histogram vec based on the supplied [Opt::Labels] or [Opt::Label] user options.
pub fn register_histogram_vec(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> Result<prometheus::HistogramVec> {
    let name = name.into();
    let mut opts = to_histogram_opts(name.clone(), help, user_opts);
    opts.common_opts
        .const_labels
        .insert(String::from("version"), crate_version!().to_string());

    let labels = opts.common_opts.variable_labels.clone();
    let labels = labels.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    register_histogram_vec!(opts, labels.as_ref()).map_err(|e| Error::from(name, e))
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use std::unimplemented;

    use super::*;

    #[test]
    fn test_counter() {
        let cnt = match register_counter("counter", "A test counter!", None) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.inc();
        assert_eq!(1.0, cnt.get());
    }

    #[test]
    fn test_counter_vec() {
        let opts = vec![Opt::Labels(vec![String::from("testing")])];
        let cnt = match register_counter_vec("counter_vec", "A test counter vec!", Some(opts)) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.with_label_values(&["woot"]).inc();
        assert_eq!(1.0, cnt.with_label_values(&["woot"]).get());
    }

    #[test]
    fn test_int_counter() {
        let cnt = match register_int_counter("int_counter", "A test int counter!", None) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.inc();
        assert_eq!(1, cnt.get());
    }

    #[test]
    fn test_int_counter_vec() {
        let opts = vec![Opt::Labels(vec![String::from("testing")])];
        let cnt = match register_int_counter_vec(
            "int_counter_vec",
            "A test int counter vec!",
            Some(opts),
        ) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.with_label_values(&["woot"]).inc();
        assert_eq!(1, cnt.with_label_values(&["woot"]).get());
    }

    #[test]
    fn test_gauge() {
        let cnt = match register_gauge("gauge", "A test gauge!", None) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.inc();
        assert_eq!(1.0, cnt.get());
    }

    #[test]
    fn test_gauge_vec() {
        let opts = vec![Opt::Labels(vec![String::from("testing")])];
        let cnt = match register_gauge_vec("gauge_vec", "A test gauge vec!", Some(opts)) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.with_label_values(&["woot"]).inc();
        assert_eq!(1.0, cnt.with_label_values(&["woot"]).get());
    }

    #[test]
    fn test_int_gauge() {
        let cnt = match register_int_gauge("int_gauge", "A test int gauge!", None) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.inc();
        assert_eq!(1, cnt.get());
    }

    #[test]
    fn test_int_gauge_vec() {
        let opts = vec![Opt::Labels(vec![String::from("testing")])];
        let cnt = match register_int_gauge_vec("int_gauge_vec", "A test int gauge vec!", Some(opts))
        {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        cnt.with_label_values(&["woot"]).inc();
        assert_eq!(1, cnt.with_label_values(&["woot"]).get());
    }

    #[test]
    fn test_histogram() {
        let hist = match register_histogram("histogram", "A test histogram!", None) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        hist.observe(0.1);
        assert_eq!(0.1, hist.get_sample_sum());
    }

    #[test]
    fn test_histogram_vec() {
        let opts = vec![Opt::Labels(vec![String::from("testing")])];
        let hist = match register_histogram_vec("histogram_vec", "A test histogram!", Some(opts)) {
            Ok(metric) => metric,
            Err(_) => unimplemented!(),
        };
        hist.with_label_values(&["woot"]).observe(0.1);
        assert_eq!(0.1, hist.with_label_values(&["woot"]).get_sample_sum());
    }
}
