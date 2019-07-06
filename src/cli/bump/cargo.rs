// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::cargo::Manifest;
use crate::cli::{bump, BumpCargoArgs, BumpCargoSetArgs, Output};
use crate::io;
use crate::Result;
use std::fs::File;

pub(crate) mod major {
    use crate::{cli::BumpCargoArgs, Result};

    pub(crate) fn run(args: BumpCargoArgs) -> Result<()> {
        super::run_major(args)
    }
}

pub(crate) mod minor {
    use crate::{cli::BumpCargoArgs, Result};

    pub(crate) fn run(args: BumpCargoArgs) -> Result<()> {
        super::run_minor(args)
    }
}

pub(crate) mod patch {
    use crate::{cli::BumpCargoArgs, Result};

    pub(crate) fn run(args: BumpCargoArgs) -> Result<()> {
        super::run_patch(args)
    }
}

pub(crate) mod set {
    use crate::{cli::BumpCargoSetArgs, Result};

    pub(crate) fn run(args: BumpCargoSetArgs) -> Result<()> {
        super::run_set(args)
    }
}

fn run_major(mut args: BumpCargoArgs) -> Result<()> {
    let mut manifest = io::read_manifest(&mut io::bufreader(&args.input)?)?;
    let version = bump::prepare_version(|| manifest.version(), args.pre, args.build)?.bump_major();
    manifest.set_version(&version);

    write_to_dest(&mut args.output, &manifest)
}

fn run_minor(mut args: BumpCargoArgs) -> Result<()> {
    let mut manifest = io::read_manifest(&mut io::bufreader(&args.input)?)?;
    let version = bump::prepare_version(|| manifest.version(), args.pre, args.build)?.bump_minor();
    manifest.set_version(&version);

    write_to_dest(&mut args.output, &manifest)
}

fn run_patch(mut args: BumpCargoArgs) -> Result<()> {
    let mut manifest = io::read_manifest(&mut io::bufreader(&args.input)?)?;
    let version = bump::prepare_version(|| manifest.version(), args.pre, args.build)?.bump_patch();
    manifest.set_version(&version);

    write_to_dest(&mut args.output, &manifest)
}

fn run_set(mut args: BumpCargoSetArgs) -> Result<()> {
    let mut manifest = io::read_manifest(&mut io::bufreader(&args.input)?)?;
    let version = bump::set_version(manifest.version()?, args.set)?;
    manifest.set_version(&version);

    write_to_dest(&mut args.output, &manifest)
}

fn write_to_dest(output: &mut Output, manifest: &Manifest) -> Result<()> {
    match output {
        Output::Stdout(writer) => io::write_manifest(writer, manifest),
        Output::File(path) => {
            // TODO: make more robust
            let mut file = File::create(path)?;
            io::write_manifest(&mut file, manifest)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::bump::test_helpers::{OutputReceiver, WriteableSender};
    use crate::cli::{
        BuildMetadata, BumpCargoArgs, BumpCargoSetArgs, Output, PreRelease, SetBuildMetadata,
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
            $name:ident, $input:expr, $output_version:expr
        ) => {
            #[test]
            fn $name() {
                let (args, tempfile) = new_args($input, None, None);
                run(args).unwrap();
                let manifest = crate::cargo::Manifest::from_str(
                    &std::fs::read_to_string(tempfile.path()).expect("file should be openable"),
                )
                .expect("manifest should be parseable");
                let version = manifest.version().expect("version should be readable");

                assert_eq!(version.to_string(), $output_version.to_string());
            }
        };
    }

    macro_rules! test_stdout {
        (
            $name:ident, $input:expr, $output_version:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output, tempfile) = new_args_stdout($input, None, None);
                run(args).unwrap();
                let manifest = crate::cargo::Manifest::from_str(&output.into_string())
                    .expect("manifest should be parseable");
                let version = manifest.version().expect("version should be readable");

                assert_eq!(version.to_string(), $output_version.to_string());

                let manifest = crate::cargo::Manifest::from_str(
                    &std::fs::read_to_string(tempfile.path()).expect("file should be openable"),
                )
                .expect("manifest should be parseable");
                let manifest_version = manifest.version().expect("version should be readable");

                assert_eq!(manifest_version.to_string(), $input);
            }
        };
    }

    macro_rules! test_set {
        (
            $name:ident, $input:expr, $major:expr, $minor:expr, $patch:expr, $version:expr,
            $pre:expr, $build:expr, $no_pre_release:expr, $no_build_metadata:expr,
            $output_version:expr
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
                let manifest = crate::cargo::Manifest::from_str(
                    &std::fs::read_to_string(tempfile.path()).expect("file should be openable"),
                )
                .expect("manifest should be parseable");
                let version = manifest.version().expect("version should be readable");

                assert_eq!(version.to_string(), $output_version.to_string());
            }
        };
    }

    macro_rules! test_set_stdout {
        (
            $name:ident, $input:expr, $major:expr, $minor:expr, $patch:expr, $version:expr,
            $pre:expr, $build:expr, $no_pre_release:expr, $no_build_metadata:expr,
            $output_version:expr
        ) => {
            #[test]
            fn $name() {
                let (args, output, tempfile) = new_set_args_stdout(
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
                let manifest = crate::cargo::Manifest::from_str(&output.into_string())
                    .expect("manifest should be parseable");
                let version = manifest.version().expect("version should be readable");

                assert_eq!(version.to_string(), $output_version.to_string());

                let manifest = crate::cargo::Manifest::from_str(
                    &std::fs::read_to_string(tempfile.path()).expect("file should be openable"),
                )
                .expect("manifest should be parseable");
                let manifest_version = manifest.version().expect("version should be readable");

                assert_eq!(manifest_version.to_string(), $input);
            }
        };
    }

    mod major {
        use super::super::major::run;
        use super::*;

        test!(round_trip, "1.0.0", "2.0.0");

        test_stdout!(stdout_round_trip, "1.0.0", "2.0.0");
    }

    mod minor {
        use super::super::minor::run;
        use super::*;

        test!(round_trip, "0.1.0", "0.2.0");

        test_stdout!(stdout_round_trip, "0.1.0", "0.2.0");
    }

    mod patch {
        use super::super::patch::run;
        use super::*;

        test!(round_trip, "0.0.1", "0.0.2");

        test_stdout!(stdout_round_trip, "0.0.1", "0.0.2");
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
            "9.2.3"
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
            "1.9.3"
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
            "1.2.9"
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
            "9.9.9"
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
            "1.2.3-pre2"
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
            "1.2.3+build8"
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
            "1.2.3"
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
            "1.2.3"
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
            "9.2.3"
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
            "1.9.3"
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
            "1.2.9"
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
            "9.9.9"
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
            "1.2.3-pre2"
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
            "1.2.3+build8"
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
            "1.2.3"
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
            "1.2.3"
        );
    }

    fn new_args<S: AsRef<str>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
    ) -> (BumpCargoArgs, NamedTempFile) {
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
        fs::write(
            tempfile.path(),
            format!("[package]\nversion = \"{}\"\n", input.as_ref()),
        )
        .expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let output = Output::File(input.clone());

        let args = BumpCargoArgs {
            pre,
            build,
            input,
            output,
        };

        (args, tempfile)
    }

    fn new_args_stdout<S: AsRef<str>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
    ) -> (BumpCargoArgs, OutputReceiver, NamedTempFile) {
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
        fs::write(
            tempfile.path(),
            format!("[package]\nversion = \"{}\"\n", input.as_ref()),
        )
        .expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let (sender, receiver) = mpsc::channel();
        let output = Output::Stdout(Box::new(WriteableSender::new(sender)));

        let args = BumpCargoArgs {
            pre,
            build,
            input,
            output,
        };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver, tempfile)
    }

    #[allow(clippy::too_many_arguments)]
    fn new_set_args<S: AsRef<str>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        version: Option<&str>,
        no_pre_release: bool,
        no_build_metadata: bool,
    ) -> (BumpCargoSetArgs, NamedTempFile) {
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
        fs::write(
            tempfile.path(),
            format!("[package]\nversion = \"{}\"\n", input.as_ref()),
        )
        .expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let output = Output::File(input.clone());

        let args = BumpCargoSetArgs { input, output, set };

        (args, tempfile)
    }

    #[allow(clippy::too_many_arguments)]
    fn new_set_args_stdout<S: AsRef<str>>(
        input: S,
        pre: Option<&str>,
        build: Option<&str>,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        version: Option<&str>,
        no_pre_release: bool,
        no_build_metadata: bool,
    ) -> (BumpCargoSetArgs, OutputReceiver, NamedTempFile) {
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
        fs::write(
            tempfile.path(),
            format!("[package]\nversion = \"{}\"\n", input.as_ref()),
        )
        .expect("input file content should be written");
        let input = PathBuf::from(tempfile.path());
        let (sender, receiver) = mpsc::channel();
        let output = Output::Stdout(Box::new(WriteableSender::new(sender)));

        let args = BumpCargoSetArgs { input, output, set };
        let output_receiver = OutputReceiver::new(receiver);

        (args, output_receiver, tempfile)
    }
}
