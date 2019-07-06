// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use log::{debug, info};
use std::process;
use std::result;
use structopt::StructOpt;

mod cli;
mod util;

/// Result type alias, using `Failure` to wrap up contexts and causes
type Result<T> = result::Result<T, failure::Error>;

fn main() {
    util::setup_panic_hooks();

    if let Err(err) = try_main() {
        // A pipe error occurs when the consumer of this process's output has hung up. This is a
        // normal event and we should quit gracefully.
        if util::is_pipe_error(&err) {
            info!("pipe error, quitting gracefully");
            process::exit(0);
        }

        // Print the error and all of its underlying causes
        eprintln!("{}", util::pretty_error(&err));

        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = cli::Args::from_args();
    util::init_logger(args.verbosity());
    debug!("parsed cli arguments; args={:?}", args);

    println!("Hello, world!");
    Ok(())
}
