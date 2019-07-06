// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use structopt::clap::AppSettings;
use structopt::StructOpt;

/// The "about" string for help messages.
const ABOUT: &str = concat!(
    "
TODO: description one-liner.

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

/// The "long_about" string for help messages.
const LONG_ABOUT: &str = concat!(
    "
TODO: description one-liner.

TODO: longer description

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

/// The parsed CLI arguments.
///
/// This struct also doubles as the CLI parser.
#[derive(Debug, StructOpt)]
#[structopt(raw(
    setting = "AppSettings::UnifiedHelpMessage",
    max_term_width = "100",
    about = "ABOUT",
    long_about = "LONG_ABOUT",
    version = "BuildInfo::version_short()",
    long_version = "BuildInfo::version_long()"
))]
pub(crate) struct Args {
    /// Verbose mode.
    ///
    /// Causes versio to print debugging messages about its progress. This is helpful
    /// when debugging problems.
    ///
    /// Multiple -v options increase the verbosity. The maximum is 3.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

impl Args {
    /// Returns the verbosity level.
    ///
    /// A `0` value is "off", and increasing numbers increase verbosity. Any value above `3` will
    /// be treated as identical to `3`.
    pub(crate) fn verbosity(&self) -> usize {
        self.verbose
    }
}

/// Build time metadata
struct BuildInfo;

impl BuildInfo {
    fn version_short() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/version_short.txt"))
    }

    fn version_long() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/version_long.txt"))
    }
}
