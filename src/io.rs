// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use crate::cargo::Manifest;
use crate::version::Version;
use crate::Result;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::path::Path;
use std::str::FromStr;

pub fn bufreader(path: &Path) -> Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn read_version<R: Read>(reader: &mut R) -> Result<Version> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(Version::from_str(buf.trim())?)
}

pub fn read_manifest<R: Read>(reader: &mut R) -> Result<Manifest> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(Manifest::from_str(&buf)?)
}

pub fn write_version<W: Write>(writer: &mut W, version: &Version) -> Result<()> {
    Ok(writeln!(writer, "{}", version)?)
}

pub fn write_manifest<W: Write>(writer: &mut W, manifest: &Manifest) -> Result<()> {
    std::io::copy(&mut Cursor::new(manifest.to_string()), writer)?;

    Ok(())
}
