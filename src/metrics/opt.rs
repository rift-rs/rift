// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

/// A metric option to use during registration.
pub enum Opt {
    /// A set of constant key/values for this metric.
    ConstLabels(HashMap<String, String>),
    /// A single constant key/value to append for this metric.
    ConstLabel(String, String),
    /// A list of variable keys for this metric.
    Labels(Vec<String>),
    /// A single variable key to append for this metric.
    Label(String),
    /// A list of buckets to use with histograms. Note this [Opt]
    /// is ignored in all cases other than histograms and summaries.
    Buckets(Vec<f64>),
    /// The namespace this metric belongs to.
    Namespace(String),
    /// The subsystem this metric belongs to.
    Subsystem(String),
}

impl Opt {
    /// Generate a new [Opt::Buckets] option using the supplied starting point, factor
    /// and count to generate exponentially increasing buckets.
    ///
    /// ```
    /// # use librift::metrics::Opt;
    ///
    /// let buckets = Opt::exponential_buckets(1.0, 3.0, 3);
    /// assert!(matches!(buckets, Opt::Buckets(_)));
    /// let buckets = match buckets {
    ///     Opt::Buckets(buckets) => buckets,
    ///     _ => unimplemented!(),
    /// };
    ///
    /// assert_eq!(buckets.len(), 3);
    /// assert_eq!(buckets[0], 1.0);
    /// assert_eq!(buckets[1], 3.0);
    /// assert_eq!(buckets[2], 9.0);
    /// ```
    pub fn exponential_buckets(start: f64, factor: f64, count: usize) -> Self {
        let mut v = Vec::with_capacity(count);
        let mut current = start;

        for _ in 0..count {
            v.push(current);
            current *= factor;
        }
        Self::Buckets(v)
    }

    /// Generate a new [Opt::Buckets] option using the supplied step, and count to
    /// generate linearly increasing buckets.
    ///
    /// ```
    /// # use librift::metrics::Opt;
    ///
    /// let buckets = Opt::linear_buckets(1.0, 3);
    /// assert!(matches!(buckets, Opt::Buckets(_)));
    /// let buckets = match buckets {
    ///     Opt::Buckets(buckets) => buckets,
    ///     _ => unimplemented!(),
    /// };
    ///
    /// assert_eq!(buckets.len(), 3);
    /// assert_eq!(buckets[0], 1.0);
    /// assert_eq!(buckets[1], 2.0);
    /// assert_eq!(buckets[2], 3.0);
    /// ```
    pub fn linear_buckets(step: f64, count: usize) -> Self {
        let mut v = Vec::with_capacity(count);
        let mut current: f64 = step;

        for _ in 0..count {
            v.push(current);
            current += step;
        }
        Self::Buckets(v)
    }
}

pub(super) fn to_common_opts(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> prometheus::Opts {
    let mut opts = prometheus::Opts::new(name, help);
    let mut user_opts = match user_opts {
        Some(user_opts) => user_opts,
        None => return opts,
    };

    for opt in user_opts.drain(..) {
        use Opt::*;
        match opt {
            ConstLabels(const_labels) => opts.const_labels = const_labels,
            ConstLabel(key, value) => {
                opts.const_labels.insert(key, value);
            }
            Labels(labels) => opts.variable_labels = labels,
            Label(label) => {
                opts.variable_labels.push(label);
            }
            Buckets(_) => continue,
            Namespace(namespace) => opts.namespace = namespace,
            Subsystem(subsystem) => opts.subsystem = subsystem,
        };
    }
    opts
}

pub(super) fn to_histogram_opts(
    name: impl Into<String>,
    help: impl Into<String>,
    user_opts: Option<Vec<Opt>>,
) -> prometheus::HistogramOpts {
    let mut opts = prometheus::HistogramOpts::new(name, help);
    let mut user_opts = match user_opts {
        Some(user_opts) => user_opts,
        None => return opts,
    };
    for opt in user_opts.drain(..) {
        use Opt::*;
        match opt {
            ConstLabels(const_labels) => opts.common_opts.const_labels = const_labels,
            ConstLabel(key, value) => {
                opts.common_opts.const_labels.insert(key, value);
            }
            Labels(labels) => opts.common_opts.variable_labels = labels,
            Label(label) => {
                opts.common_opts.variable_labels.push(label);
            }
            Buckets(buckets) => opts.buckets = buckets,
            Namespace(namespace) => opts.common_opts.namespace = namespace,
            Subsystem(subsystem) => opts.common_opts.subsystem = subsystem,
        };
    }
    opts
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_buckets() {
        let buckets = Opt::exponential_buckets(1.0, 3.0, 3);
        assert!(matches!(buckets, Opt::Buckets(_)));
        let buckets = match buckets {
            Opt::Buckets(buckets) => buckets,
            _ => unimplemented!(),
        };

        assert_eq!(buckets.len(), 3);
        assert_eq!(buckets[0], 1.0);
        assert_eq!(buckets[1], 3.0);
        assert_eq!(buckets[2], 9.0);
    }

    #[test]
    fn test_linear_buckets() {
        let buckets = Opt::linear_buckets(1.0, 3);
        assert!(matches!(buckets, Opt::Buckets(_)));
        let buckets = match buckets {
            Opt::Buckets(buckets) => buckets,
            _ => unimplemented!(),
        };

        assert_eq!(buckets.len(), 3);
        assert_eq!(buckets[0], 1.0);
        assert_eq!(buckets[1], 2.0);
        assert_eq!(buckets[2], 3.0);
    }

    #[test]
    fn test_to_common_opts_no_user_opts() {
        let name = String::from("name");
        let help = String::from("help");

        let opts = to_common_opts(name.clone(), help.clone(), None);
        assert!(opts.namespace.is_empty());
        assert!(opts.subsystem.is_empty());
        assert_eq!(opts.name, name);
        assert_eq!(opts.help, help);
        assert!(opts.const_labels.is_empty());
        assert!(opts.variable_labels.is_empty());
    }

    #[test]
    fn test_to_common_opts_user_opts() {
        let name = String::from("name");
        let help = String::from("help");
        let namespace = String::from("namespace");
        let subsystem = String::from("subsystem");

        let const_key = String::from("key");
        let const_val = String::from("val");
        let mut const_labels = HashMap::with_capacity(1);
        const_labels.insert(const_key.clone(), const_val.clone());

        let const_key_2 = String::from("key2");
        let const_val_2 = String::from("val2");

        let variable_key = String::from("variable");
        let mut variable_lables = Vec::with_capacity(1);
        variable_lables.push(variable_key.clone());

        let variable_key_2 = String::from("variable2");

        let user_opts = vec![
            Opt::Namespace(namespace.clone()),
            Opt::Subsystem(subsystem.clone()),
            Opt::ConstLabels(const_labels),
            Opt::ConstLabel(const_key_2.clone(), const_val_2.clone()),
            Opt::Labels(variable_lables),
            Opt::Label(variable_key_2.clone()),
            Opt::Buckets(Vec::new()),
        ];

        let opts = to_common_opts(name.clone(), help.clone(), Some(user_opts));
        assert_eq!(opts.namespace, namespace);
        assert_eq!(opts.subsystem, subsystem);
        assert_eq!(opts.name, name);
        assert_eq!(opts.help, help);
        assert_eq!(opts.const_labels.len(), 2);

        let val = opts
            .const_labels
            .get(&const_key)
            .expect("key missing for const_labels");
        assert_eq!(val, &const_val);

        let val = opts
            .const_labels
            .get(&const_key_2)
            .expect("key missing from const_labels");
        assert_eq!(val, &const_val_2);

        assert_eq!(opts.variable_labels.len(), 2);
        assert_eq!(opts.variable_labels[0], variable_key);
        assert_eq!(opts.variable_labels[1], variable_key_2);
    }

    #[test]
    fn test_to_histogram_opts_no_user_opts() {
        let name = String::from("name");
        let help = String::from("help");

        let opts = to_histogram_opts(name.clone(), help.clone(), None);
        assert!(opts.common_opts.namespace.is_empty());
        assert!(opts.common_opts.subsystem.is_empty());
        assert_eq!(opts.common_opts.name, name);
        assert_eq!(opts.common_opts.help, help);
        assert!(opts.common_opts.const_labels.is_empty());
        assert!(opts.common_opts.variable_labels.is_empty());
        assert_eq!(opts.buckets.len(), 11);
    }

    #[test]
    fn test_to_histogram_opts_user_opts() {
        let name = String::from("name");
        let help = String::from("help");
        let namespace = String::from("namespace");
        let subsystem = String::from("subsystem");

        let const_key = String::from("key");
        let const_val = String::from("val");
        let mut const_labels = HashMap::with_capacity(1);
        const_labels.insert(const_key.clone(), const_val.clone());

        let const_key_2 = String::from("key2");
        let const_val_2 = String::from("val2");

        let variable_key = String::from("variable");
        let mut variable_lables = Vec::with_capacity(1);
        variable_lables.push(variable_key.clone());

        let variable_key_2 = String::from("variable2");

        let user_opts = vec![
            Opt::Namespace(namespace.clone()),
            Opt::Subsystem(subsystem.clone()),
            Opt::ConstLabels(const_labels),
            Opt::ConstLabel(const_key_2.clone(), const_val_2.clone()),
            Opt::Labels(variable_lables),
            Opt::Label(variable_key_2.clone()),
            Opt::Buckets(vec![1.0, 2.0, 3.0]),
        ];

        let opts = to_histogram_opts(name.clone(), help.clone(), Some(user_opts));
        assert_eq!(opts.common_opts.namespace, namespace);
        assert_eq!(opts.common_opts.subsystem, subsystem);
        assert_eq!(opts.common_opts.name, name);
        assert_eq!(opts.common_opts.help, help);
        assert_eq!(opts.common_opts.const_labels.len(), 2);

        let val = opts
            .common_opts
            .const_labels
            .get(&const_key)
            .expect("key missing for const_labels");
        assert_eq!(val, &const_val);

        let val = opts
            .common_opts
            .const_labels
            .get(&const_key_2)
            .expect("key missing from const_labels");
        assert_eq!(val, &const_val_2);

        assert_eq!(opts.common_opts.variable_labels.len(), 2);
        assert_eq!(opts.common_opts.variable_labels[0], variable_key);
        assert_eq!(opts.common_opts.variable_labels[1], variable_key_2);

        assert_eq!(opts.buckets.len(), 3);
        assert_eq!(opts.buckets[0], 1.0);
        assert_eq!(opts.buckets[1], 2.0);
        assert_eq!(opts.buckets[2], 3.0);
    }
}
