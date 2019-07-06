// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::cli::{bump, BumpFileArgs, BumpFileSetArgs, Output};
use crate::io;
use crate::version::Version;
use crate::Result;
use std::fs::File;

pub(crate) mod major {
    use crate::{cli::BumpFileArgs, Result};

    pub(crate) fn run(args: BumpFileArgs) -> Result<()> {
        super::run_major(args)
    }
}

pub(crate) mod minor {
    use crate::{cli::BumpFileArgs, Result};

    pub(crate) fn run(args: BumpFileArgs) -> Result<()> {
        super::run_minor(args)
    }
}

pub(crate) mod patch {
    use crate::{cli::BumpFileArgs, Result};

    pub(crate) fn run(args: BumpFileArgs) -> Result<()> {
        super::run_patch(args)
    }
}

pub(crate) mod set {
    use crate::{cli::BumpFileSetArgs, Result};

    pub(crate) fn run(args: BumpFileSetArgs) -> Result<()> {
        super::run_set(args)
    }
}

fn run_major(mut args: BumpFileArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut io::bufreader(&args.input)?, args.pre, args.build)?
            .bump_major();

    write_to_dest(&mut args.output, &version)
}

fn run_minor(mut args: BumpFileArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut io::bufreader(&args.input)?, args.pre, args.build)?
            .bump_minor();

    write_to_dest(&mut args.output, &version)
}

fn run_patch(mut args: BumpFileArgs) -> Result<()> {
    let version =
        bump::prepare_version_from_reader(&mut io::bufreader(&args.input)?, args.pre, args.build)?
            .bump_patch();

    write_to_dest(&mut args.output, &version)
}

fn run_set(mut args: BumpFileSetArgs) -> Result<()> {
    let version = bump::set_version(
        io::read_version(&mut io::bufreader(&args.input)?)?,
        args.set,
    )?;

    write_to_dest(&mut args.output, &version)
}

fn write_to_dest(output: &mut Output, version: &Version) -> Result<()> {
    match output {
        Output::Stdout(writer) => io::write_version(writer, version),
        Output::File(path) => {
            // TODO: make more robust
            let mut file = File::create(path)?;
            io::write_version(&mut file, version)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::bump::test_helpers::{OutputReceiver, WriteableSender};
    use crate::cli::{
        BuildMetadata, BumpFileArgs, BumpFileSetArgs, Output, PreRelease, SetBuildMetadata,
        SetPreRelease, SetVersion,
    };
    use crate::version::Version;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::mpsc;
    use tempfile::NamedTempFile;

    macro_rules! test {
        (
            $name:ident, $input:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, tempfile) = new_args($input, None, None);
                run(args).unwrap();
                let output = String::from_utf8(
                    std::fs::read(tempfile.path()).expect("file contents should be read"),
                )
                .expect("content should be valid utf-8");

                assert_eq!(output, $output);
            }
        };
    }

    macro_rules! test_stdout {
        (
            $name:ident, $input:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output, _tempfile) = new_args_stdout($input, None, None);
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
                let (args, tempfile) = new_args($input, $pre, $build);
                run(args).unwrap();
                let output = String::from_utf8(
                    std::fs::read(tempfile.path()).expect("file contents should be read"),
                )
                .expect("content should be valid utf-8");

                assert_eq!(output, $output);
            }
        };
    }

    macro_rules! test_with_extras_stdout {
        (
            $name:ident, $input:expr, $pre:expr, $build:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output, _tempfile) = new_args_stdout($input, $pre, $build);
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
                let (args, tempfile) = new_set_args(
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
                let output = String::from_utf8(
                    std::fs::read(tempfile.path()).expect("file contents should be read"),
                )
                .expect("content should be valid utf-8");

                assert_eq!(output, $output);
            }
        };
    }

    macro_rules! test_set_stdout {
        (
            $name:ident, $input:expr, $major:expr, $minor:expr, $patch:expr, $version:expr,
            $pre:expr, $build:expr, $no_pre_release:expr, $no_build_metadata:expr, $output:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output, _tempfile) = new_set_args_stdout(
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

        test_stdout!(stdout_leading_whitespace, "    1.0.0", "2.0.0\n");
        test_stdout!(stdout_trailing_whitespace, "1.0.0    ", "2.0.0\n");
        test_stdout!(stdout_both_whitespace, "    1.0.0    ", "2.0.0\n");
        test_stdout!(stdout_newline, "1.0.0\n", "2.0.0\n");
        test_stdout!(stdout_multiple_newlines, "1.0.0\n\n\n\n\n", "2.0.0\n");
        test_stdout!(stdout_leading_newline, "\n1.0.0", "2.0.0\n");
        test_stdout!(stdout_both_newlines, "\n\n1.0.0\n\n\n", "2.0.0\n");
        test_stdout!(
            stdout_mixed_newlines_and_whitespace,
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

        test_with_extras_stdout!(
            stdout_with_pre,
            "1.0.0",
            Some("alpha"),
            None,
            "2.0.0-alpha\n"
        );
        test_with_extras_stdout!(
            stdout_with_build,
            "1.0.0",
            None,
            Some("build8"),
            "2.0.0+build8\n"
        );
        test_with_extras_stdout!(
            stdout_with_pre_and_build,
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

        test_stdout!(stdout_leading_whitespace, "    0.1.0", "0.2.0\n");
        test_stdout!(stdout_trailing_whitespace, "0.1.0    ", "0.2.0\n");
        test_stdout!(stdout_both_whitespace, "    0.1.0    ", "0.2.0\n");
        test_stdout!(stdout_newline, "0.1.0\n", "0.2.0\n");
        test_stdout!(stdout_multiple_newlines, "0.1.0\n\n\n\n\n", "0.2.0\n");
        test_stdout!(stdout_leading_newline, "\n0.1.0", "0.2.0\n");
        test_stdout!(stdout_both_newlines, "\n\n0.1.0\n\n\n", "0.2.0\n");
        test_stdout!(
            stdout_mixed_newlines_and_whitespace,
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

        test_with_extras_stdout!(
            stdout_with_pre,
            "0.1.0",
            Some("alpha"),
            None,
            "0.2.0-alpha\n"
        );
        test_with_extras_stdout!(
            stdout_with_build,
            "0.1.0",
            None,
            Some("build8"),
            "0.2.0+build8\n"
        );
        test_with_extras_stdout!(
            stdout_with_pre_and_build,
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

        test_stdout!(stdout_leading_whitespace, "    0.0.1", "0.0.2\n");
        test_stdout!(stdout_trailing_whitespace, "0.0.1    ", "0.0.2\n");
        test_stdout!(stdout_both_whitespace, "    0.0.1    ", "0.0.2\n");
        test_stdout!(stdout_newline, "0.0.1\n", "0.0.2\n");
        test_stdout!(stdout_multiple_newlines, "0.0.1\n\n\n\n\n", "0.0.2\n");
        test_stdout!(stdout_leading_newline, "\n0.0.1", "0.0.2\n");
        test_stdout!(stdout_both_newlines, "\n\n0.0.1\n\n\n", "0.0.2\n");
        test_stdout!(
            stdout_mixed_newlines_and_whitespace,
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

        test_with_extras_stdout!(
            stdout_with_pre,
            "0.0.1",
            Some("alpha"),
            None,
            "0.0.2-alpha\n"
        );
        test_with_extras_stdout!(
            stdout_with_build,
            "0.0.1",
            None,
            Some("build8"),
            "0.0.2+build8\n"
        );
        test_with_extras_stdout!(
            stdout_with_pre_and_build,
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

        test_set_stdout!(
            stdout_major,
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
        test_set_stdout!(
            stdout_minor,
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
        test_set_stdout!(
            stdout_patch,
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
        test_set_stdout!(
            stdout_version,
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
        test_set_stdout!(
            stdout_pre,
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
        test_set_stdout!(
            stdout_build,
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
        test_set_stdout!(
            stdout_no_pre,
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
        test_set_stdout!(
            stdout_no_build,
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
    ) -> (BumpFileArgs, NamedTempFile) {
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
        let tempfile = NamedTempFile::new().expect("tempfile should be created");
        fs::write(tempfile.path(), input.into()).expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let output = Output::File(input.clone());

        let args = BumpFileArgs {
            pre,
            build,
            input,
            output,
        };

        (args, tempfile)
    }

    fn new_args_stdout<S: Into<String>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
    ) -> (BumpFileArgs, OutputReceiver, NamedTempFile) {
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
        let tempfile = NamedTempFile::new().expect("tempfile should be created");
        fs::write(tempfile.path(), input.into()).expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let (sender, receiver) = mpsc::channel();
        let output = Output::Stdout(Box::new(WriteableSender::new(sender)));

        let args = BumpFileArgs {
            pre,
            build,
            input,
            output,
        };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver, tempfile)
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
    ) -> (BumpFileSetArgs, NamedTempFile) {
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
        let tempfile = NamedTempFile::new().expect("tempfile should be created");
        fs::write(tempfile.path(), input.into()).expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let output = Output::File(input.clone());

        let args = BumpFileSetArgs { input, output, set };

        (args, tempfile)
    }

    #[allow(clippy::too_many_arguments)]
    fn new_set_args_stdout<S: Into<String>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        version: Option<&str>,
        no_pre_release: bool,
        no_build_metadata: bool,
    ) -> (BumpFileSetArgs, OutputReceiver, NamedTempFile) {
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
        let tempfile = NamedTempFile::new().expect("tempfile should be created");
        fs::write(tempfile.path(), input.into()).expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let (sender, receiver) = mpsc::channel();
        let output = Output::Stdout(Box::new(WriteableSender::new(sender)));

        let args = BumpFileSetArgs { input, output, set };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver, tempfile)
    }
}
