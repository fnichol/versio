// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::cli::{SetBuildMetadata, SetPreRelease, SetVersion};
use crate::io;
use crate::version::{BuildMetadata, PreRelease, Version, VersionBumper};
use crate::Result;
use std::io::Read;

pub(crate) mod cargo;
pub(crate) mod file;
pub(crate) mod stdin;

fn prepare_version_from_reader<R: Read>(
    reader: &mut R,
    pre: Option<PreRelease>,
    build: Option<BuildMetadata>,
) -> Result<VersionBumper> {
    prepare_version(|| io::read_version(reader), pre, build)
}

fn prepare_version<F>(
    mut version: F,
    pre: Option<PreRelease>,
    build: Option<BuildMetadata>,
) -> Result<VersionBumper>
where
    F: FnMut() -> Result<Version>,
{
    Ok(VersionBumper::new(version()?)
        .maybe_pre(pre)
        .maybe_build(build))
}

fn set_version(initial_version: Version, set: SetVersion) -> Result<Version> {
    let v = match set {
        SetVersion::Version(version) => version,
        SetVersion::Parts {
            major,
            minor,
            patch,
            pre,
            build,
        } => {
            let mut bumper = VersionBumper::new(initial_version);

            if let Some(major) = major {
                bumper = bumper.major(major);
            }
            if let Some(minor) = minor {
                bumper = bumper.minor(minor);
            }
            if let Some(patch) = patch {
                bumper = bumper.patch(patch);
            }
            match pre {
                SetPreRelease::Some(pre) => bumper = bumper.pre(pre),
                SetPreRelease::Clear => bumper = bumper.clear_pre(),
                SetPreRelease::None => {}
            }
            match build {
                SetBuildMetadata::Some(build) => bumper = bumper.build(build),
                SetBuildMetadata::Clear => bumper = bumper.clear_build(),
                SetBuildMetadata::None => {}
            }

            bumper.no_bump()
        }
    };

    Ok(v)
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::io;
    use std::sync::mpsc::{Receiver, Sender};

    pub(crate) struct WriteableSender(Sender<String>);

    impl WriteableSender {
        pub(crate) fn new(sender: Sender<String>) -> Self {
            WriteableSender(sender)
        }
    }

    impl io::Write for WriteableSender {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .send(String::from_utf8_lossy(buf).to_owned().to_string())
                .expect("should send full slice");
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    pub(crate) struct OutputReceiver(Receiver<String>);

    impl OutputReceiver {
        pub(crate) fn new(receiver: Receiver<String>) -> Self {
            OutputReceiver(receiver)
        }

        pub(crate) fn into_string(self) -> String {
            self.into()
        }
    }

    impl From<OutputReceiver> for String {
        fn from(output: OutputReceiver) -> Self {
            let mut buf = String::new();
            while let Ok(ref content) = output.0.recv() {
                buf.push_str(content);
            }
            buf
        }
    }
}
