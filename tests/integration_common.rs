// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use common::{help_bin_name, BIN_NAME};

mod common;

include!("common/macros.rs");

mod base {
    use super::*;

    basic_cmd_behavior!(BIN_NAME);
}

mod cmd_bump {
    use super::*;

    basic_cmd_behavior!(format!("{}-bump", help_bin_name()), "bump");
}

mod cmd_bump_cargo {
    use super::*;

    basic_cmd_behavior!(format!("{}-bump-cargo", help_bin_name()), "bump", "cargo");
}

mod cmd_bump_cargo_major {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-cargo-major", help_bin_name()),
        "bump",
        "cargo",
        "major"
    );
}

mod cmd_bump_cargo_minor {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-cargo-minor", help_bin_name()),
        "bump",
        "cargo",
        "minor"
    );
}

mod cmd_bump_cargo_patch {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-cargo-patch", help_bin_name()),
        "bump",
        "cargo",
        "patch"
    );
}

mod cmd_bump_cargo_set {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-cargo-set", help_bin_name()),
        "bump",
        "cargo",
        "set"
    );
}

mod cmd_bump_file {
    use super::*;

    basic_cmd_behavior!(format!("{}-bump-file", help_bin_name()), "bump", "file");
}

mod cmd_bump_file_major {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-file-major", help_bin_name()),
        "bump",
        "file",
        "major"
    );
}

mod cmd_bump_file_minor {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-file-minor", help_bin_name()),
        "bump",
        "file",
        "minor"
    );
}

mod cmd_bump_file_patch {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-file-patch", help_bin_name()),
        "bump",
        "file",
        "patch"
    );
}

mod cmd_bump_file_set {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-file-set", help_bin_name()),
        "bump",
        "file",
        "set"
    );
}

mod cmd_bump_stdin {
    use super::*;

    basic_cmd_behavior!(format!("{}-bump-stdin", help_bin_name()), "bump", "stdin");
}

mod cmd_bump_stdin_major {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-stdin-major", help_bin_name()),
        "bump",
        "stdin",
        "major"
    );
}

mod cmd_bump_stdin_minor {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-stdin-minor", help_bin_name()),
        "bump",
        "stdin",
        "minor"
    );
}

mod cmd_bump_stdin_patch {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-stdin-patch", help_bin_name()),
        "bump",
        "stdin",
        "patch"
    );
}

mod cmd_bump_stdin_set {
    use super::*;

    basic_cmd_behavior!(
        format!("{}-bump-stdin-set", help_bin_name()),
        "bump",
        "stdin",
        "set"
    );
}
