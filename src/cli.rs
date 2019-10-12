// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::version::{BuildMetadata, PreRelease, Version};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::clap::AppSettings::{InferSubcommands, UnifiedHelpMessage, VersionlessSubcommands};
use structopt::StructOpt;

pub(crate) mod bump;
pub(crate) mod util;

const AUTHOR: &str = concat!(env!("CARGO_PKG_AUTHORS"), "\n\n");

/// The "about" string for help messages.
const ABOUT: &str = concat!(
    "\
TODO: description one-liner.

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

/// The "long_about" string for help messages.
const LONG_ABOUT: &str = concat!(
    "\
TODO: description one-liner.

TODO: longer description

Project home page: ",
    env!("CARGO_PKG_HOMEPAGE"),
    r"

Use -h for short descriptions and --help for more details.",
);

/// The parsed CLI arguments.
///
/// This struct also doubles as the CLI parser.
#[derive(Debug, StructOpt)]
#[structopt(raw(
    global_settings = "&[UnifiedHelpMessage, InferSubcommands, VersionlessSubcommands]",
    max_term_width = "100",
    author = "AUTHOR",
    about = "ABOUT",
    long_about = "LONG_ABOUT",
    version = "BuildInfo::version_short()",
    long_version = "BuildInfo::version_long()"
))]
pub(super) struct Args {
    /// Verbose mode.
    ///
    /// Causes versio to print debugging messages about its progress. This is helpful
    /// when debugging problems.
    ///
    /// Multiple -v options increase the verbosity. The maximum is 3.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences), global = true)]
    verbose: usize,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

impl Args {
    pub(crate) fn subcmd(self) -> SubCommand {
        self.subcmd
    }

    /// Returns the verbosity level.
    ///
    /// A `0` value is "off", and increasing numbers increase verbosity. Any value above `3` will
    /// be treated as identical to `3`.
    pub(crate) fn verbosity(&self) -> usize {
        self.verbose
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(super) enum SubCommand {
    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Bump {
        #[structopt(subcommand)]
        subcmd: BumpSubCommand,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(super) enum BumpSubCommand {
    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Cargo {
        #[structopt(subcommand)]
        subcmd: BumpCargoSubCommand,
    },

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    File {
        #[structopt(subcommand)]
        subcmd: BumpFileSubCommand,
    },

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Stdin {
        #[structopt(subcommand)]
        subcmd: BumpStdinSubCommand,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(super) enum BumpCargoSubCommand {
    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Major(CliBumpCargoArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Minor(CliBumpCargoArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Patch(CliBumpCargoArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Set(CliBumpCargoSetArgs),
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(super) enum BumpFileSubCommand {
    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Major(CliBumpFileArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Minor(CliBumpFileArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Patch(CliBumpFileArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Set(CliBumpFileSetArgs),
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(super) enum BumpStdinSubCommand {
    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Major(CliBumpStdinArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Minor(CliBumpStdinArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Patch(CliBumpStdinArgs),

    /// TODO: description.
    #[structopt(raw(author = "AUTHOR"))]
    Set(CliBumpStdinSetArgs),
}

#[derive(Debug, StructOpt)]
pub(super) struct BumpCommonArgs {
    /// TODO: description.
    #[structopt(long = "pre-release", short = "P", rename_all = "screaming_snake_case")]
    pre_release: Option<PreRelease>,

    /// TODO: description.
    #[structopt(
        long = "build-metadata",
        short = "b",
        rename_all = "screaming_snake_case"
    )]
    build_metadata: Option<BuildMetadata>,
}

#[derive(Debug, StructOpt)]
pub(super) struct BumpSetArgs {
    /// TODO: description.
    #[structopt(short = "M", long = "major", rename_all = "screaming_snake_case")]
    major: Option<u64>,

    /// TODO: description.
    #[structopt(short = "m", long = "minor", rename_all = "screaming_snake_case")]
    minor: Option<u64>,

    /// TODO: description.
    #[structopt(short = "p", long = "patch", rename_all = "screaming_snake_case")]
    patch: Option<u64>,

    /// TODO: description.
    #[structopt(
        short = "V",
        long = "version",
        rename_all = "screaming_snake_case",
        raw(
            conflicts_with_all = r#"&["MAJOR", "MINOR", "PATCH", "PRE_RELEASE", "BUILD_METADATA"]"#
        )
    )]
    version: Option<Version>,

    /// TODO: description.
    #[structopt(
        long = "no-pre-release",
        raw(conflicts_with_all = r#"&["PRE_RELEASE"]"#)
    )]
    no_pre_release: bool,

    /// TODO: description.
    #[structopt(
        long = "no-build-metadata",
        raw(conflicts_with_all = r#"&["BUILD_METADATA"]"#)
    )]
    no_build_metadata: bool,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpCargoArgs {
    #[structopt(flatten)]
    common: BumpCommonArgs,

    /// TODO: description.
    #[structopt(
        rename_all = "screaming_snake_case",
        raw(default_value = "\"Cargo.toml\"")
    )]
    manifest: PathBuf,

    /// TODO: description.
    #[structopt(short = "s", long)]
    stdout: bool,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpCargoSetArgs {
    #[structopt(flatten)]
    pub common: CliBumpCargoArgs,

    #[structopt(flatten)]
    pub set: BumpSetArgs,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpFileArgs {
    #[structopt(flatten)]
    common: BumpCommonArgs,

    /// TODO: description.
    #[structopt(
        rename_all = "screaming_snake_case",
        raw(default_value = "\"VERSION.txt\"")
    )]
    file: PathBuf,

    /// TODO: description.
    #[structopt(short = "s", long)]
    stdout: bool,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpFileSetArgs {
    #[structopt(flatten)]
    common: CliBumpFileArgs,

    #[structopt(flatten)]
    set: BumpSetArgs,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpStdinArgs {
    #[structopt(flatten)]
    common: BumpCommonArgs,
}

#[derive(Debug, StructOpt)]
pub(super) struct CliBumpStdinSetArgs {
    #[structopt(flatten)]
    common: CliBumpStdinArgs,

    #[structopt(flatten)]
    set: BumpSetArgs,
}

/// Build time metadata
struct BuildInfo;

impl BuildInfo {
    fn version_short() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/version_short.txt"))
    }

    fn version_long() -> &'static str {
        include_str!(concat!(env!("OUT_DIR"), "/version_long.txt"))
    }
}

pub(crate) enum Output {
    File(PathBuf),
    Stdout(Box<dyn Write>),
}

pub(crate) struct BumpCargoArgs {
    pub pre: Option<PreRelease>,
    pub build: Option<BuildMetadata>,
    pub input: PathBuf,
    pub output: Output,
}

impl From<CliBumpCargoArgs> for BumpCargoArgs {
    fn from(args: CliBumpCargoArgs) -> Self {
        BumpCargoArgs {
            pre: args.common.pre_release,
            build: args.common.build_metadata,
            input: args.manifest.clone(),
            output: if args.stdout {
                Output::Stdout(Box::new(std::io::stdout()))
            } else {
                Output::File(args.manifest)
            },
        }
    }
}

pub(crate) struct BumpCargoSetArgs {
    pub input: PathBuf,
    pub output: Output,
    pub set: SetVersion,
}

impl From<CliBumpCargoSetArgs> for BumpCargoSetArgs {
    fn from(args: CliBumpCargoSetArgs) -> Self {
        BumpCargoSetArgs {
            input: args.common.manifest.clone(),
            output: if args.common.stdout {
                Output::Stdout(Box::new(std::io::stdout()))
            } else {
                Output::File(args.common.manifest.clone())
            },
            set: args.into(),
        }
    }
}

pub(crate) struct BumpFileArgs {
    pub pre: Option<PreRelease>,
    pub build: Option<BuildMetadata>,
    pub input: PathBuf,
    pub output: Output,
}

impl From<CliBumpFileArgs> for BumpFileArgs {
    fn from(args: CliBumpFileArgs) -> Self {
        BumpFileArgs {
            pre: args.common.pre_release,
            build: args.common.build_metadata,
            input: args.file.clone(),
            output: if args.stdout {
                Output::Stdout(Box::new(std::io::stdout()))
            } else {
                Output::File(args.file)
            },
        }
    }
}

pub(crate) struct BumpFileSetArgs {
    pub input: PathBuf,
    pub output: Output,
    pub set: SetVersion,
}

impl From<CliBumpFileSetArgs> for BumpFileSetArgs {
    fn from(args: CliBumpFileSetArgs) -> Self {
        BumpFileSetArgs {
            input: args.common.file.clone(),
            output: if args.common.stdout {
                Output::Stdout(Box::new(std::io::stdout()))
            } else {
                Output::File(args.common.file.clone())
            },
            set: args.into(),
        }
    }
}

pub(crate) struct BumpStdinArgs {
    pub pre: Option<PreRelease>,
    pub build: Option<BuildMetadata>,
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
}

impl From<CliBumpStdinArgs> for BumpStdinArgs {
    fn from(args: CliBumpStdinArgs) -> Self {
        BumpStdinArgs {
            pre: args.common.pre_release,
            build: args.common.build_metadata,
            input: Box::new(std::io::stdin()),
            output: Box::new(std::io::stdout()),
        }
    }
}

pub(crate) struct BumpStdinSetArgs {
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
    pub set: SetVersion,
}

impl From<CliBumpStdinSetArgs> for BumpStdinSetArgs {
    fn from(args: CliBumpStdinSetArgs) -> Self {
        BumpStdinSetArgs {
            input: Box::new(std::io::stdin()),
            output: Box::new(std::io::stdout()),
            set: args.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SetPreRelease {
    Some(PreRelease),
    Clear,
    None,
}

#[derive(Debug)]
pub(crate) enum SetBuildMetadata {
    Some(BuildMetadata),
    Clear,
    None,
}

#[derive(Debug)]
pub(crate) enum SetVersion {
    Version(Version),
    Parts {
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: SetPreRelease,
        build: SetBuildMetadata,
    },
}

impl From<CliBumpCargoSetArgs> for SetVersion {
    fn from(args: CliBumpCargoSetArgs) -> Self {
        match args.set.version {
            Some(version) => SetVersion::Version(version),
            None => SetVersion::Parts {
                major: args.set.major,
                minor: args.set.minor,
                patch: args.set.patch,
                pre: if args.set.no_pre_release {
                    SetPreRelease::Clear
                } else {
                    match args.common.common.pre_release {
                        Some(pre) => SetPreRelease::Some(pre),
                        None => SetPreRelease::None,
                    }
                },
                build: if args.set.no_build_metadata {
                    SetBuildMetadata::Clear
                } else {
                    match args.common.common.build_metadata {
                        Some(build) => SetBuildMetadata::Some(build),
                        None => SetBuildMetadata::None,
                    }
                },
            },
        }
    }
}

impl From<CliBumpFileSetArgs> for SetVersion {
    fn from(args: CliBumpFileSetArgs) -> Self {
        match args.set.version {
            Some(version) => SetVersion::Version(version),
            None => SetVersion::Parts {
                major: args.set.major,
                minor: args.set.minor,
                patch: args.set.patch,
                pre: if args.set.no_pre_release {
                    SetPreRelease::Clear
                } else {
                    match args.common.common.pre_release {
                        Some(pre) => SetPreRelease::Some(pre),
                        None => SetPreRelease::None,
                    }
                },
                build: if args.set.no_build_metadata {
                    SetBuildMetadata::Clear
                } else {
                    match args.common.common.build_metadata {
                        Some(build) => SetBuildMetadata::Some(build),
                        None => SetBuildMetadata::None,
                    }
                },
            },
        }
    }
}

impl From<CliBumpStdinSetArgs> for SetVersion {
    fn from(args: CliBumpStdinSetArgs) -> Self {
        match args.set.version {
            Some(version) => SetVersion::Version(version),
            None => SetVersion::Parts {
                major: args.set.major,
                minor: args.set.minor,
                patch: args.set.patch,
                pre: if args.set.no_pre_release {
                    SetPreRelease::Clear
                } else {
                    match args.common.common.pre_release {
                        Some(pre) => SetPreRelease::Some(pre),
                        None => SetPreRelease::None,
                    }
                },
                build: if args.set.no_build_metadata {
                    SetBuildMetadata::Clear
                } else {
                    match args.common.common.build_metadata {
                        Some(build) => SetBuildMetadata::Some(build),
                        None => SetBuildMetadata::None,
                    }
                },
            },
        }
    }
}
