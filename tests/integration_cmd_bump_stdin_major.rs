// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

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
