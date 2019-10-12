// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use log::{debug, info};
use std::process;
use std::result;
use structopt::StructOpt;

mod cargo;
mod cli;
mod io;
mod version;

/// Result type alias, using `Failure` to wrap up contexts and causes
type Result<T> = result::Result<T, failure::Error>;

fn main() {
    cli::util::setup_panic_hooks();

    if let Err(err) = try_main() {
        // A pipe error occurs when the consumer of this process's output has hung up. This is a
        // normal event and we should quit gracefully.
        if cli::util::is_pipe_error(&err) {
            info!("pipe error, quitting gracefully");
            process::exit(0);
        }

        // Print the error and all of its underlying causes
        eprintln!("{}", cli::util::pretty_error(&err));

        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = cli::Args::from_args();
    cli::util::init_logger(args.verbosity());
    debug!("parsed cli arguments; args={:?}", args);

    use cli::SubCommand::*;

    match args.subcmd() {
        Bump { subcmd } => {
            use cli::BumpSubCommand::*;

            match subcmd {
                Cargo { subcmd } => {
                    use cli::bump::cargo;
                    use cli::BumpCargoSubCommand::*;

                    match subcmd {
                        Major(args) => cargo::major::run(args.into()),
                        Minor(args) => cargo::minor::run(args.into()),
                        Patch(args) => cargo::patch::run(args.into()),
                        Set(args) => cargo::set::run(args.into()),
                    }
                }
                File { subcmd } => {
                    use cli::bump::file;
                    use cli::BumpFileSubCommand::*;

                    match subcmd {
                        Major(args) => file::major::run(args.into()),
                        Minor(args) => file::minor::run(args.into()),
                        Patch(args) => file::patch::run(args.into()),
                        Set(args) => file::set::run(args.into()),
                    }
                }
                Stdin { subcmd } => {
                    use cli::bump::stdin;
                    use cli::BumpStdinSubCommand::*;

                    match subcmd {
                        Major(args) => stdin::major::run(args.into()),
                        Minor(args) => stdin::minor::run(args.into()),
                        Patch(args) => stdin::patch::run(args.into()),
                        Set(args) => stdin::set::run(args.into()),
                    }
                }
            }
        }
    }
}
