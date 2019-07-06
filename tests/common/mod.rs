// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use assert_cmd::prelude::*;
use assert_cmd::{assert::Assert, crate_name};
use predicates::prelude::*;
use std::process::Command;

pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const BIN_NAME: &str = crate_name!();
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const CHECK_FOR: &str = "TODO";

#[allow(dead_code)]
pub fn cmd() -> Command {
    let mut cmd = match Command::cargo_bin(BIN_NAME) {
        Ok(cmd) => cmd,
        Err(e) => panic!(
            "Cannot find Command::cargo_bin({:?}), err={:?}",
            BIN_NAME, e
        ),
    };
    cmd.current_dir("tests/fixtures");
    cmd
}

#[cfg(not(target_family = "windows"))]
#[allow(dead_code)]
pub fn help_bin_name() -> &'static str {
    "versio"
}

#[cfg(target_family = "windows")]
#[allow(dead_code)]
pub fn help_bin_name() -> &'static str {
    "versio.exe"
}

#[allow(dead_code)]
pub fn assert_cmd_output_no_todos(cmd: Assert) {
    cmd.stderr(predicate::str::contains(CHECK_FOR).not())
        .stdout(predicate::str::contains(CHECK_FOR).not());
}

#[allow(dead_code)]
pub fn assert_help_short<S: AsRef<str>>(mut cmd: Command, cmd_help_str: S) {
    cmd.arg("-h").assert().success().stderr("").stdout(
        predicate::str::starts_with(format!("{} {}", cmd_help_str.as_ref(), VERSION))
            // Ensure there is a blank line after the authors
            .and(predicate::str::contains(format!("{}\n\n", AUTHOR)))
            // Ensure that the usage is displayed
            .and(predicate::str::contains("USAGE:\n")),
    );
}

#[allow(dead_code)]
pub fn assert_help_long<S: AsRef<str>>(mut cmd: Command, cmd_help_str: S) {
    cmd.arg("--help").assert().success().stderr("").stdout(
        predicate::str::starts_with(format!("{} {}", cmd_help_str.as_ref(), VERSION))
            // Ensure there is a blank line after the authors
            .and(predicate::str::contains(format!("{}\n\n", AUTHOR)))
            // Ensure that the usage is displayed
            .and(predicate::str::contains("USAGE:\n"))
            // Check that the long descriptions are displayed for options, flags, and arguments
            .and(predicate::str::contains(
                "Multiple -v options increase the verbosity. The maximum is 3.",
            )),
    );
}
