// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;

mod common;

include!("common/macros.rs");

cmd!("bump", "stdin", "major");

fn cmd_with_stdin<S>(buffer: S) -> Assert
where
    S: Into<Vec<u8>>,
{
    cmd().with_stdin().buffer(buffer).assert()
}

#[test]
fn simple_no_nl() {
    cmd_with_stdin("0.0.0")
        .success()
        .stderr("")
        .stdout("1.0.0\n");
}

#[test]
fn simple_nl() {
    cmd_with_stdin("0.0.0\n")
        .success()
        .stderr("")
        .stdout("1.0.0\n");
}
