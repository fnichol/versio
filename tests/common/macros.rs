// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
