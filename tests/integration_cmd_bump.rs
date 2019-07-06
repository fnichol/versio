// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

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
