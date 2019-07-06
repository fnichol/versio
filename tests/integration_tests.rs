// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const BIN_NAME: &str = crate_name!();
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.current_dir("tests");
    cmd
}

#[test]
fn version_short() {
    cmd()
        .arg("-V")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(predicate::str::contains(format!("binary: {}", BIN_NAME)).not())
                .and(predicate::str::contains(format!("release: {}", VERSION)).not()),
        )
        .stderr("");
}

#[test]
fn version_long() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(predicate::str::contains(format!("binary: {}", BIN_NAME)))
                .and(predicate::str::contains(format!("release: {}", VERSION))),
        )
        .stderr("");
}
