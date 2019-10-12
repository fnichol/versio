// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use assert_cmd::prelude::*;
use predicate::str;
use predicates::prelude::*;

mod common;

include!("common/macros.rs");

cmd!("bump");

#[test]
fn no_args() {
    cmd()
        .assert()
        .failure()
        .stdout("")
        .stderr(str::contains("USAGE:\n").and(str::contains("SUBCOMMANDS:\n")));
}
