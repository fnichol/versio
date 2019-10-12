// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use assert_cmd::prelude::*;
use common::{cmd, BIN_NAME, VERSION};
use predicate::str;
use predicates::prelude::*;

mod common;

const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

#[test]
fn help_short_extra_info() {
    cmd().arg("-h").assert().success().stderr("").stdout(
        // Ensure that the project home page is displayed
        str::contains(format!("Project home page: {}\n\n", HOMEPAGE))
            // Ensure that a message is displayed for the more detailed help
            .and(str::contains(
                "Use -h for short descriptions and --help for more details.\n\n",
            )),
    );
}

#[test]
fn help_long_extra_info() {
    cmd().arg("--help").assert().success().stderr("").stdout(
        // Ensure that the project home page is displayed
        str::contains(format!("Project home page: {}\n\n", HOMEPAGE))
            // Ensure that a message is displayed for the more detailed help
            .and(str::contains(
                "Use -h for short descriptions and --help for more details.\n\n",
            )),
    );
}

#[test]
fn version_short() {
    cmd()
        .arg("-V")
        .assert()
        .success()
        .stdout(
            str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(str::contains(format!("binary: {}", BIN_NAME)).not())
                .and(str::contains(format!("release: {}", VERSION)).not()),
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
            str::starts_with(format!("{} {}", BIN_NAME, VERSION))
                .and(str::contains(format!("binary: {}", BIN_NAME)))
                .and(str::contains(format!("release: {}", VERSION))),
        )
        .stderr("");
}
#[test]
fn no_args() {
    cmd()
        .assert()
        .failure()
        .stdout("")
        .stderr(str::contains("USAGE:\n").and(str::contains("SUBCOMMANDS:\n")));
}
