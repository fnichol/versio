// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

#[allow(unused_macros)]
macro_rules! basic_cmd_behavior {
    (
        $prog_help_str:expr
    ) => {
        cmd!();

        __impl_basic_cmd_tests!($prog_help_str);
    };

    (
        $prog_help_str:expr,
        $(
            $subcmd:expr
        ),
        +
    ) => {
        cmd!($($subcmd),+);

        __impl_basic_cmd_tests!($prog_help_str);
    };
}

#[allow(unused_macros)]
macro_rules! cmd {
    (
    ) => {
        fn cmd() -> std::process::Command {
            common::cmd()
        }
    };

    (
        $(
            $subcmd:expr
        ),
        +
    ) => {
        fn cmd() -> std::process::Command {
            let mut cmd = common::cmd();
            $(
                cmd.arg(format!("{}", $subcmd));
            )*
            cmd
        }
    };
}

#[allow(unused_macros)]
macro_rules! __impl_basic_cmd_tests {
    (
        $prog_help_str:expr
    ) => {
        #[test]
        fn help_short() {
            common::assert_help_short(cmd(), format!("{}", $prog_help_str));
        }

        #[test]
        fn help_long() {
            common::assert_help_long(cmd(), format!("{}", $prog_help_str));
        }

        // #[test]
        // fn help_short_no_todos() {
        //     use assert_cmd::assert::OutputAssertExt;

        //     common::assert_cmd_output_no_todos(cmd().arg("-h").assert().success())
        // }

        // #[test]
        // fn help_long_no_todos() {
        //     use assert_cmd::assert::OutputAssertExt;

        //     common::assert_cmd_output_no_todos(cmd().arg("--help").assert().success())
        // }
    };
}
