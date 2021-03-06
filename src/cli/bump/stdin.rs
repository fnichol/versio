// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::{bump, BumpStdinArgs, BumpStdinSetArgs};
use crate::io;
use crate::version::Version;
use crate::Result;
use std::io::Write;

pub(crate) mod major {
    use crate::{cli::BumpStdinArgs, Result};

    pub(crate) fn run(args: BumpStdinArgs) -> Result<()> {
        super::run_major(args)
    }
}

pub(crate) mod minor {
    use crate::{cli::BumpStdinArgs, Result};

    pub(crate) fn run(args: BumpStdinArgs) -> Result<()> {
        super::run_minor(args)
    }
}

pub(crate) mod patch {
    use crate::{cli::BumpStdinArgs, Result};

    pub(crate) fn run(args: BumpStdinArgs) -> Result<()> {
        super::run_patch(args)
    }
}

pub(crate) mod set {
    use crate::{cli::BumpStdinSetArgs, Result};

    pub(crate) fn run(args: BumpStdinSetArgs) -> Result<()> {
        super::run_set(args)
    }
}

fn run_major(mut args: BumpStdinArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut args.input, args.pre, args.build)?.bump_major();

    write_to_dest(&mut args.output, &version)
}

fn run_minor(mut args: BumpStdinArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut args.input, args.pre, args.build)?.bump_minor();

    write_to_dest(&mut args.output, &version)
}

fn run_patch(mut args: BumpStdinArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut args.input, args.pre, args.build)?.bump_patch();

    write_to_dest(&mut args.output, &version)
}

fn run_set(mut args: BumpStdinSetArgs) -> Result<()> {
    let version = bump::set_version(io::read_version(&mut args.input)?, args.set)?;

    write_to_dest(&mut args.output, &version)
}

fn write_to_dest<W: Write>(writer: &mut W, version: &Version) -> Result<()> {
    io::write_version(writer, version)
}

#[cfg(test)]
mod tests {
    use crate::cli::bump::test_helpers::{OutputReceiver, WriteableSender};
    use crate::cli::{
        BuildMetadata, BumpStdinArgs, BumpStdinSetArgs, PreRelease, SetBuildMetadata,
        SetPreRelease, SetVersion,
    };
    use crate::version::Version;
    use std::io::Cursor;
    use std::str::FromStr;
    use std::sync::mpsc;

    macro_rules! test {
        (
            $name:ident, $input:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output) = new_args($input, None, None);
                run(args).unwrap();

                assert_eq!(output.into_string(), $output);
            }
        };
    }

    macro_rules! test_with_extras {
        (
            $name:ident, $input:expr, $pre:expr, $build:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output) = new_args($input, $pre, $build);
                run(args).unwrap();

                assert_eq!(output.into_string(), $output);
            }
        };
    }

    macro_rules! test_set {
        (
            $name:ident, $input:expr, $major:expr, $minor:expr, $patch:expr, $version:expr,
            $pre:expr, $build:expr, $no_pre_release:expr, $no_build_metadata:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output) = new_set_args(
                    $input,
                    $pre,
                    $build,
                    $major,
                    $minor,
                    $patch,
                    $version,
                    $no_pre_release,
                    $no_build_metadata,
                );
                run(args).unwrap();

                assert_eq!(output.into_string(), $output);
            }
        };
    }

    mod major {
        use super::super::major::run;
        use super::*;

        test!(leading_whitespace, "    1.0.0", "2.0.0\n");
        test!(trailing_whitespace, "1.0.0    ", "2.0.0\n");
        test!(both_whitespace, "    1.0.0    ", "2.0.0\n");
        test!(newline, "1.0.0\n", "2.0.0\n");
        test!(multiple_newlines, "1.0.0\n\n\n\n\n", "2.0.0\n");
        test!(leading_newline, "\n1.0.0", "2.0.0\n");
        test!(both_newlines, "\n\n1.0.0\n\n\n", "2.0.0\n");
        test!(
            mixed_newlines_and_whitespace,
            "  \n \n\n  1.0.0   \n \n ",
            "2.0.0\n"
        );

        test_with_extras!(with_pre, "1.0.0", Some("alpha"), None, "2.0.0-alpha\n");
        test_with_extras!(with_build, "1.0.0", None, Some("build8"), "2.0.0+build8\n");
        test_with_extras!(
            with_pre_and_build,
            "1.0.0",
            Some("beta"),
            Some("build8"),
            "2.0.0-beta+build8\n"
        );
    }

    mod minor {
        use super::super::minor::run;
        use super::*;

        test!(leading_whitespace, "    0.1.0", "0.2.0\n");
        test!(trailing_whitespace, "0.1.0    ", "0.2.0\n");
        test!(both_whitespace, "    0.1.0    ", "0.2.0\n");
        test!(newline, "0.1.0\n", "0.2.0\n");
        test!(multiple_newlines, "0.1.0\n\n\n\n\n", "0.2.0\n");
        test!(leading_newline, "\n0.1.0", "0.2.0\n");
        test!(both_newlines, "\n\n0.1.0\n\n\n", "0.2.0\n");
        test!(
            mixed_newlines_and_whitespace,
            "  \n \n\n  0.1.0   \n \n ",
            "0.2.0\n"
        );

        test_with_extras!(with_pre, "0.1.0", Some("alpha"), None, "0.2.0-alpha\n");
        test_with_extras!(with_build, "0.1.0", None, Some("build8"), "0.2.0+build8\n");
        test_with_extras!(
            with_pre_and_build,
            "0.1.0",
            Some("beta"),
            Some("build8"),
            "0.2.0-beta+build8\n"
        );
    }

    mod patch {
        use super::super::patch::run;
        use super::*;

        test!(leading_whitespace, "    0.0.1", "0.0.2\n");
        test!(trailing_whitespace, "0.0.1    ", "0.0.2\n");
        test!(both_whitespace, "    0.0.1    ", "0.0.2\n");
        test!(newline, "0.0.1\n", "0.0.2\n");
        test!(multiple_newlines, "0.0.1\n\n\n\n\n", "0.0.2\n");
        test!(leading_newline, "\n0.0.1", "0.0.2\n");
        test!(both_newlines, "\n\n0.0.1\n\n\n", "0.0.2\n");
        test!(
            mixed_newlines_and_whitespace,
            "  \n \n\n  0.0.1   \n \n ",
            "0.0.2\n"
        );

        test_with_extras!(with_pre, "0.0.1", Some("alpha"), None, "0.0.2-alpha\n");
        test_with_extras!(with_build, "0.0.1", None, Some("build8"), "0.0.2+build8\n");
        test_with_extras!(
            with_pre_and_build,
            "0.0.1",
            Some("beta"),
            Some("build8"),
            "0.0.2-beta+build8\n"
        );
    }

    mod set {
        use super::super::set::run;
        use super::*;

        test_set!(
            major,
            "1.2.3",
            Some(9),
            None,
            None,
            None,
            None,
            None,
            false,
            false,
            "9.2.3\n"
        );
        test_set!(
            minor,
            "1.2.3",
            None,
            Some(9),
            None,
            None,
            None,
            None,
            false,
            false,
            "1.9.3\n"
        );
        test_set!(
            patch,
            "1.2.3",
            None,
            None,
            Some(9),
            None,
            None,
            None,
            false,
            false,
            "1.2.9\n"
        );
        test_set!(
            version,
            "1.2.3",
            None,
            None,
            None,
            Some("9.9.9"),
            None,
            None,
            false,
            false,
            "9.9.9\n"
        );
        test_set!(
            pre,
            "1.2.3",
            None,
            None,
            None,
            None,
            Some("pre2"),
            None,
            false,
            false,
            "1.2.3-pre2\n"
        );
        test_set!(
            build,
            "1.2.3",
            None,
            None,
            None,
            None,
            None,
            Some("build8"),
            false,
            false,
            "1.2.3+build8\n"
        );
        test_set!(
            no_pre,
            "1.2.3-rc9",
            None,
            None,
            None,
            None,
            None,
            None,
            true,
            false,
            "1.2.3\n"
        );
        test_set!(
            no_build,
            "1.2.3+build8",
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            true,
            "1.2.3\n"
        );
    }

    fn new_args<S: Into<String>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
    ) -> (BumpStdinArgs, OutputReceiver) {
        let pre = match pre {
            Some(pre_str) => Some(PreRelease::from_str(pre_str).expect("should be valid pre")),
            None => None,
        };
        let build = match build {
            Some(build_str) => {
                Some(BuildMetadata::from_str(build_str).expect("should be valid build"))
            }
            None => None,
        };
        let (sender, receiver) = mpsc::channel();

        let args = BumpStdinArgs {
            pre,
            build,
            input: Box::new(Cursor::new(input.into())),
            output: Box::new(WriteableSender::new(sender)),
        };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver)
    }

    #[allow(clippy::too_many_arguments)]
    fn new_set_args<S: Into<String>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        version: Option<&str>,
        no_pre_release: bool,
        no_build_metadata: bool,
    ) -> (BumpStdinSetArgs, OutputReceiver) {
        let set = if let Some(version) = version {
            SetVersion::Version(Version::from_str(version).expect("should be valid version"))
        } else {
            SetVersion::Parts {
                major,
                minor,
                patch,
                pre: if no_pre_release {
                    SetPreRelease::Clear
                } else {
                    match pre {
                        Some(pre_str) => SetPreRelease::Some(
                            PreRelease::from_str(pre_str).expect("should be valid pre"),
                        ),
                        None => SetPreRelease::None,
                    }
                },
                build: if no_build_metadata {
                    SetBuildMetadata::Clear
                } else {
                    match build {
                        Some(build_str) => SetBuildMetadata::Some(
                            BuildMetadata::from_str(build_str).expect("should be valid build"),
                        ),
                        None => SetBuildMetadata::None,
                    }
                },
            }
        };
        let (sender, receiver) = mpsc::channel();

        let args = BumpStdinSetArgs {
            input: Box::new(Cursor::new(input.into())),
            output: Box::new(WriteableSender::new(sender)),
            set,
        };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver)
    }
}
