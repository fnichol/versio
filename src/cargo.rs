// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::version::Version;
use crate::Result;
use std::fmt;
use std::result;
use std::str::FromStr;
use toml_edit::{Document, TomlError};

#[derive(Clone, Debug)]
pub struct Manifest(Document);

impl Manifest {
    pub fn version(&self) -> Result<Version> {
        let version_str = match self.0["package"]["version"].as_str() {
            Some(version_str) => version_str,
            None => panic!("TODO: cannot find version in Cargo manifest"),
        };

        Ok(Version::from_str(version_str)?)
    }

    pub fn set_version(&mut self, version: &Version) {
        self.0["package"]["version"] = toml_edit::value(version.to_string());
    }
}

impl FromStr for Manifest {
    type Err = TomlError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(Manifest(Document::from_str(s)?))
    }
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let manifest = Manifest::from_str("[package]\nversion = \"1.2.3\"\n").unwrap();

        assert_eq!("[package]\nversion = \"1.2.3\"\n", manifest.to_string());
    }

    // #[test]
    // fn from_str_err() {
    //     match Manifest::from_str("not a toml file") {
    //         Err(TomlError) => (),
    //         Ok(m) => panic!("should not parse: {:?}", m),
    //     }
    // }

    #[test]
    fn version() {
        let manifest = Manifest::from_str("[package]\nversion = \"1.2.3\"\n").unwrap();

        assert_eq!(manifest.version().unwrap().to_string(), "1.2.3");
    }

    // #[test]
    // fn version_err() {
    //     let manifest = Manifest::from_str("[package]\nversion = \"nope.nope\"\n").unwrap();

    //     match manifest.version() {
    //         Err(e) => {
    //             dbg!(&e);
    //             panic!("err: {:?}", e);
    //         }
    //         Ok(v) => panic!("should not parse version: {:?}", v),
    //     }
    // }
}
