// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use semver::{Identifier, SemVerError};
use std::fmt;
use std::result;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct PreRelease(Vec<Identifier>);

impl FromStr for PreRelease {
    type Err = SemVerError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(PreRelease(
            Version::from_str(&format!("0.0.0-{}", s))?.0.pre,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct BuildMetadata(Vec<Identifier>);

impl FromStr for BuildMetadata {
    type Err = SemVerError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(BuildMetadata(
            Version::from_str(&format!("0.0.0+{}", s))?.0.build,
        ))
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Version(semver::Version);

impl FromStr for Version {
    type Err = SemVerError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(Version(semver::Version::from_str(s)?))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<semver::Version> for Version {
    fn from(version: semver::Version) -> Self {
        Version(version)
    }
}

#[derive(Debug)]
pub struct VersionBumper {
    version: Version,
    pre: Option<PreRelease>,
    build: Option<BuildMetadata>,
}

impl VersionBumper {
    pub fn new(version: Version) -> Self {
        VersionBumper {
            version,
            pre: None,
            build: None,
        }
    }

    pub fn maybe_pre(self, pre: Option<PreRelease>) -> Self {
        match pre {
            Some(pre) => self.pre(pre),
            None => self,
        }
    }

    pub fn pre(mut self, pre: PreRelease) -> Self {
        self.pre = Some(pre);
        self
    }

    pub fn clear_pre(mut self) -> Self {
        self.version.0.pre = Vec::new();
        self.pre = None;
        self
    }

    pub fn maybe_build(self, build: Option<BuildMetadata>) -> Self {
        match build {
            Some(build) => self.build(build),
            None => self,
        }
    }

    pub fn build(mut self, build: BuildMetadata) -> Self {
        self.build = Some(build);
        self
    }

    pub fn clear_build(mut self) -> Self {
        self.version.0.build = Vec::new();
        self.build = None;
        self
    }

    pub fn major(mut self, major: u64) -> Self {
        self.version.0.major = major;
        self
    }

    pub fn bump_major(mut self) -> Version {
        self.version.0.increment_major();
        self.consume()
    }

    pub fn minor(mut self, minor: u64) -> Self {
        self.version.0.minor = minor;
        self
    }

    pub fn bump_minor(mut self) -> Version {
        self.version.0.increment_minor();
        self.consume()
    }

    pub fn bump_patch(mut self) -> Version {
        self.version.0.increment_patch();
        self.consume()
    }

    pub fn patch(mut self, patch: u64) -> Self {
        self.version.0.patch = patch;
        self
    }

    pub fn no_bump(self) -> Version {
        self.consume()
    }

    fn consume(mut self) -> Version {
        if let Some(pre) = self.pre {
            self.version.0.pre = pre.0;
        }
        if let Some(build) = self.build {
            self.version.0.build = build.0;
        }

        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pre_release {
        use super::*;

        #[test]
        fn from_str() {
            let pre = PreRelease::from_str("alpha1").unwrap();
            let parts: Vec<_> = pre.0.iter().map(|i| i.to_string()).collect();

            assert_eq!(parts, vec!["alpha1"]);
        }

        #[test]
        fn from_str_err() {
            match PreRelease::from_str("+++uh oh...") {
                Err(SemVerError::ParseError(_)) => (),
                Ok(pre) => panic!("should not parse: {:?}", pre),
            }
        }
    }

    mod build_metadata {
        use super::*;

        #[test]
        fn from_str() {
            let build = BuildMetadata::from_str("build2").unwrap();
            let parts: Vec<_> = build.0.iter().map(|i| i.to_string()).collect();

            assert_eq!(parts, vec!["build2"]);
        }

        #[test]
        fn from_str_err() {
            match BuildMetadata::from_str("---uh oh...") {
                Err(SemVerError::ParseError(_)) => (),
                Ok(build) => panic!("should not parse: {:?}", build),
            }
        }
    }

    mod version {
        use super::*;

        #[test]
        fn from_str() {
            let version = Version::from_str("1.2.3").unwrap();

            assert_eq!(version.to_string(), "1.2.3");
        }

        #[test]
        fn from_str_err() {
            match Version::from_str("nope.nope") {
                Err(SemVerError::ParseError(_)) => (),
                Ok(version) => panic!("should not parse: {:?}", version),
            }
        }

        #[test]
        fn from_semver_version() {
            let version = semver::Version::from_str("1.2.3").unwrap();
            let native_version = Version::from_str("1.2.3").unwrap();

            assert_eq!(native_version, version.into());
        }
    }

    mod version_bumper {
        use super::*;

        fn version(version_str: &str) -> Version {
            Version::from_str(version_str).expect("version str should parse")
        }

        fn pre(pre_str: &str) -> PreRelease {
            PreRelease::from_str(pre_str).expect("pre str should parse")
        }

        fn build(build_str: &str) -> BuildMetadata {
            BuildMetadata::from_str(build_str).expect("build str should parse")
        }

        fn bumper(version_str: &str) -> VersionBumper {
            VersionBumper::new(version(version_str))
        }

        #[test]
        fn major() {
            assert_eq!(bumper("1.2.3").bump_major().to_string(), "2.0.0");
        }

        #[test]
        fn major_zero() {
            assert_eq!(bumper("0.0.0").bump_major().to_string(), "1.0.0");
        }

        #[test]
        fn major_nine_to_ten() {
            assert_eq!(bumper("9.0.0").bump_major().to_string(), "10.0.0");
        }

        #[test]
        fn major_ninety_nine_to_one_hundred() {
            assert_eq!(bumper("99.0.0").bump_major().to_string(), "100.0.0");
        }

        #[test]
        fn major_exisiting_pre() {
            assert_eq!(bumper("1.2.3-alpha").bump_major().to_string(), "2.0.0");
        }

        #[test]
        fn major_exisiting_build() {
            assert_eq!(bumper("1.2.3+build1").bump_major().to_string(), "2.0.0");
        }

        #[test]
        fn major_exisiting_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1").bump_major().to_string(),
                "2.0.0"
            );
        }

        #[test]
        fn major_and_pre() {
            assert_eq!(
                bumper("1.2.3").pre(pre("rc1")).bump_major().to_string(),
                "2.0.0-rc1"
            );
        }

        #[test]
        fn major_and_pre_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .pre(pre("rc1"))
                    .clear_pre()
                    .bump_major()
                    .to_string(),
                "2.0.0"
            );
        }

        #[test]
        fn major_and_maybe_pre_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_pre(Some(pre("rc1")))
                    .bump_major()
                    .to_string(),
                "2.0.0-rc1"
            );
        }

        #[test]
        fn major_and_maybe_pre_none() {
            assert_eq!(
                bumper("1.2.3").maybe_pre(None).bump_major().to_string(),
                "2.0.0"
            );
        }

        #[test]
        fn major_exisiting_pre_and_pre() {
            assert_eq!(
                bumper("1.2.3-alpha")
                    .pre(pre("beta"))
                    .bump_major()
                    .to_string(),
                "2.0.0-beta"
            );
        }

        #[test]
        fn major_and_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .bump_major()
                    .to_string(),
                "2.0.0+build7"
            );
        }

        #[test]
        fn major_and_build_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .clear_build()
                    .bump_major()
                    .to_string(),
                "2.0.0"
            );
        }

        #[test]
        fn major_and_maybe_build_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_build(Some(build("build7")))
                    .bump_major()
                    .to_string(),
                "2.0.0+build7"
            );
        }

        #[test]
        fn major_and_maybe_build_none() {
            assert_eq!(
                bumper("1.2.3").maybe_build(None).bump_major().to_string(),
                "2.0.0"
            );
        }

        #[test]
        fn major_exisiting_build_and_build() {
            assert_eq!(
                bumper("1.2.3+build1")
                    .build(build("yep"))
                    .bump_major()
                    .to_string(),
                "2.0.0+yep"
            );
        }

        #[test]
        fn major_and_pre_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .pre(pre("beta3"))
                    .bump_major()
                    .to_string(),
                "2.0.0-beta3+build7"
            );
        }

        #[test]
        fn major_exisiting_pre_and_build_and_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1")
                    .build(build("yep"))
                    .pre(pre("beta1"))
                    .bump_major()
                    .to_string(),
                "2.0.0-beta1+yep"
            );
        }

        #[test]
        fn major_set_major() {
            assert_eq!(bumper("1.2.3").major(2).bump_major().to_string(), "3.0.0");
        }

        #[test]
        fn major_set_minor() {
            assert_eq!(bumper("1.2.3").minor(3).bump_major().to_string(), "2.0.0");
        }

        #[test]
        fn major_set_patch() {
            assert_eq!(bumper("1.2.3").patch(4).bump_major().to_string(), "2.0.0");
        }

        #[test]
        fn minor() {
            assert_eq!(bumper("1.2.3").bump_minor().to_string(), "1.3.0");
        }

        #[test]
        fn minor_zero() {
            assert_eq!(bumper("0.0.0").bump_minor().to_string(), "0.1.0");
        }

        #[test]
        fn minor_nine_to_ten() {
            assert_eq!(bumper("0.9.0").bump_minor().to_string(), "0.10.0");
        }

        #[test]
        fn minor_ninety_nine_to_one_hundred() {
            assert_eq!(bumper("0.99.0").bump_minor().to_string(), "0.100.0");
        }

        #[test]
        fn minor_exisiting_pre() {
            assert_eq!(bumper("1.2.3-alpha").bump_minor().to_string(), "1.3.0");
        }

        #[test]
        fn minor_exisiting_build() {
            assert_eq!(bumper("1.2.3+build1").bump_minor().to_string(), "1.3.0");
        }

        #[test]
        fn minor_exisiting_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1").bump_minor().to_string(),
                "1.3.0"
            );
        }

        #[test]
        fn minor_and_pre() {
            assert_eq!(
                bumper("1.2.3").pre(pre("rc1")).bump_minor().to_string(),
                "1.3.0-rc1"
            );
        }

        #[test]
        fn minor_and_pre_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .pre(pre("rc1"))
                    .clear_pre()
                    .bump_minor()
                    .to_string(),
                "1.3.0"
            );
        }

        #[test]
        fn minor_and_maybe_pre_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_pre(Some(pre("rc1")))
                    .bump_minor()
                    .to_string(),
                "1.3.0-rc1"
            );
        }

        #[test]
        fn minor_and_maybe_pre_none() {
            assert_eq!(
                bumper("1.2.3").maybe_pre(None).bump_minor().to_string(),
                "1.3.0"
            );
        }

        #[test]
        fn minor_exisiting_pre_and_pre() {
            assert_eq!(
                bumper("1.2.3-alpha")
                    .pre(pre("beta"))
                    .bump_minor()
                    .to_string(),
                "1.3.0-beta"
            );
        }

        #[test]
        fn minor_and_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .bump_minor()
                    .to_string(),
                "1.3.0+build7"
            );
        }

        #[test]
        fn minor_and_build_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .clear_build()
                    .bump_minor()
                    .to_string(),
                "1.3.0"
            );
        }

        #[test]
        fn minor_and_maybe_build_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_build(Some(build("build7")))
                    .bump_minor()
                    .to_string(),
                "1.3.0+build7"
            );
        }

        #[test]
        fn minor_and_maybe_build_none() {
            assert_eq!(
                bumper("1.2.3").maybe_build(None).bump_minor().to_string(),
                "1.3.0"
            );
        }

        #[test]
        fn minor_exisiting_build_and_build() {
            assert_eq!(
                bumper("1.2.3+build1")
                    .build(build("yep"))
                    .bump_minor()
                    .to_string(),
                "1.3.0+yep"
            );
        }

        #[test]
        fn minor_and_pre_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .pre(pre("beta3"))
                    .bump_minor()
                    .to_string(),
                "1.3.0-beta3+build7"
            );
        }

        #[test]
        fn minor_exisiting_pre_and_build_and_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1")
                    .build(build("yep"))
                    .pre(pre("beta1"))
                    .bump_minor()
                    .to_string(),
                "1.3.0-beta1+yep"
            );
        }

        #[test]
        fn minor_set_major() {
            assert_eq!(bumper("1.2.3").major(2).bump_minor().to_string(), "2.3.0");
        }

        #[test]
        fn minor_set_minor() {
            assert_eq!(bumper("1.2.3").minor(3).bump_minor().to_string(), "1.4.0");
        }

        #[test]
        fn minor_set_patch() {
            assert_eq!(bumper("1.2.3").patch(4).bump_minor().to_string(), "1.3.0");
        }

        #[test]
        fn patch() {
            assert_eq!(bumper("1.2.3").bump_patch().to_string(), "1.2.4");
        }

        #[test]
        fn patch_zero() {
            assert_eq!(bumper("0.0.0").bump_patch().to_string(), "0.0.1");
        }

        #[test]
        fn patch_nine_to_ten() {
            assert_eq!(bumper("0.0.9").bump_patch().to_string(), "0.0.10");
        }

        #[test]
        fn patch_ninety_nine_to_one_hundred() {
            assert_eq!(bumper("0.0.99").bump_patch().to_string(), "0.0.100");
        }

        #[test]
        fn patch_exisiting_pre() {
            assert_eq!(bumper("1.2.3-alpha").bump_patch().to_string(), "1.2.4");
        }

        #[test]
        fn patch_exisiting_build() {
            assert_eq!(bumper("1.2.3+build1").bump_patch().to_string(), "1.2.4");
        }

        #[test]
        fn patch_exisiting_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1").bump_patch().to_string(),
                "1.2.4"
            );
        }

        #[test]
        fn patch_and_pre() {
            assert_eq!(
                bumper("1.2.3").pre(pre("rc1")).bump_patch().to_string(),
                "1.2.4-rc1"
            );
        }

        #[test]
        fn patch_and_pre_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .pre(pre("rc1"))
                    .clear_pre()
                    .bump_patch()
                    .to_string(),
                "1.2.4"
            );
        }

        #[test]
        fn patch_and_maybe_pre_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_pre(Some(pre("rc1")))
                    .bump_patch()
                    .to_string(),
                "1.2.4-rc1"
            );
        }

        #[test]
        fn patch_and_maybe_pre_none() {
            assert_eq!(
                bumper("1.2.3").maybe_pre(None).bump_patch().to_string(),
                "1.2.4"
            );
        }

        #[test]
        fn patch_exisiting_pre_and_pre() {
            assert_eq!(
                bumper("1.2.3-alpha")
                    .pre(pre("beta"))
                    .bump_patch()
                    .to_string(),
                "1.2.4-beta"
            );
        }

        #[test]
        fn patch_and_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .bump_patch()
                    .to_string(),
                "1.2.4+build7"
            );
        }

        #[test]
        fn patch_and_build_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .clear_build()
                    .bump_patch()
                    .to_string(),
                "1.2.4"
            );
        }

        #[test]
        fn patch_and_maybe_build_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_build(Some(build("build7")))
                    .bump_patch()
                    .to_string(),
                "1.2.4+build7"
            );
        }

        #[test]
        fn patch_and_maybe_build_none() {
            assert_eq!(
                bumper("1.2.3").maybe_build(None).bump_patch().to_string(),
                "1.2.4"
            );
        }

        #[test]
        fn patch_exisiting_build_and_build() {
            assert_eq!(
                bumper("1.2.3+build1")
                    .build(build("yep"))
                    .bump_patch()
                    .to_string(),
                "1.2.4+yep"
            );
        }

        #[test]
        fn patch_and_pre_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .pre(pre("beta3"))
                    .bump_patch()
                    .to_string(),
                "1.2.4-beta3+build7"
            );
        }

        #[test]
        fn patch_exisiting_pre_and_build_and_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1")
                    .build(build("yep"))
                    .pre(pre("beta1"))
                    .bump_patch()
                    .to_string(),
                "1.2.4-beta1+yep"
            );
        }

        #[test]
        fn patch_set_major() {
            assert_eq!(bumper("1.2.3").major(2).bump_patch().to_string(), "2.2.4");
        }

        #[test]
        fn patch_set_minor() {
            assert_eq!(bumper("1.2.3").minor(3).bump_patch().to_string(), "1.3.4");
        }

        #[test]
        fn patch_set_patch() {
            assert_eq!(bumper("1.2.3").patch(4).bump_patch().to_string(), "1.2.5");
        }

        #[test]
        fn no_bump() {
            assert_eq!(bumper("1.2.3").no_bump().to_string(), "1.2.3");
        }

        #[test]
        fn no_bump_zero() {
            assert_eq!(bumper("0.0.0").no_bump().to_string(), "0.0.0");
        }

        #[test]
        fn no_bump_exisiting_pre() {
            assert_eq!(bumper("1.2.3-alpha").no_bump().to_string(), "1.2.3-alpha");
        }

        #[test]
        fn no_bump_exisiting_build() {
            assert_eq!(bumper("1.2.3+build1").no_bump().to_string(), "1.2.3+build1");
        }

        #[test]
        fn no_bump_exisiting_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1").no_bump().to_string(),
                "1.2.3-alpha+build1"
            );
        }

        #[test]
        fn no_bump_and_pre() {
            assert_eq!(
                bumper("1.2.3").pre(pre("rc1")).no_bump().to_string(),
                "1.2.3-rc1"
            );
        }

        #[test]
        fn no_bump_and_pre_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .pre(pre("rc1"))
                    .clear_pre()
                    .no_bump()
                    .to_string(),
                "1.2.3"
            );
        }

        #[test]
        fn no_bump_and_maybe_pre_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_pre(Some(pre("rc1")))
                    .no_bump()
                    .to_string(),
                "1.2.3-rc1"
            );
        }

        #[test]
        fn no_bump_and_maybe_pre_none() {
            assert_eq!(
                bumper("1.2.3").maybe_pre(None).no_bump().to_string(),
                "1.2.3"
            );
        }

        #[test]
        fn no_bump_exisiting_pre_and_pre() {
            assert_eq!(
                bumper("1.2.3-alpha").pre(pre("beta")).no_bump().to_string(),
                "1.2.3-beta"
            );
        }

        #[test]
        fn no_bump_and_build() {
            assert_eq!(
                bumper("1.2.3").build(build("build7")).no_bump().to_string(),
                "1.2.3+build7"
            );
        }

        #[test]
        fn no_bump_and_build_and_clear() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .clear_build()
                    .no_bump()
                    .to_string(),
                "1.2.3"
            );
        }

        #[test]
        fn no_bump_and_maybe_build_some() {
            assert_eq!(
                bumper("1.2.3")
                    .maybe_build(Some(build("build7")))
                    .no_bump()
                    .to_string(),
                "1.2.3+build7"
            );
        }

        #[test]
        fn no_bump_and_maybe_build_none() {
            assert_eq!(
                bumper("1.2.3").maybe_build(None).no_bump().to_string(),
                "1.2.3"
            );
        }

        #[test]
        fn no_bump_exisiting_build_and_build() {
            assert_eq!(
                bumper("1.2.3+build1")
                    .build(build("yep"))
                    .no_bump()
                    .to_string(),
                "1.2.3+yep"
            );
        }

        #[test]
        fn no_bump_and_pre_build() {
            assert_eq!(
                bumper("1.2.3")
                    .build(build("build7"))
                    .pre(pre("beta3"))
                    .no_bump()
                    .to_string(),
                "1.2.3-beta3+build7"
            );
        }

        #[test]
        fn no_bump_exisiting_pre_and_build_and_pre_and_build() {
            assert_eq!(
                bumper("1.2.3-alpha+build1")
                    .build(build("yep"))
                    .pre(pre("beta1"))
                    .no_bump()
                    .to_string(),
                "1.2.3-beta1+yep"
            );
        }

        #[test]
        fn no_bump_set_major() {
            assert_eq!(bumper("1.2.3").major(2).no_bump().to_string(), "2.2.3");
        }

        #[test]
        fn no_bump_set_minor() {
            assert_eq!(bumper("1.2.3").minor(3).no_bump().to_string(), "1.3.3");
        }

        #[test]
        fn no_bump_set_patch() {
            assert_eq!(bumper("1.2.3").patch(4).no_bump().to_string(), "1.2.4");
        }
    }
}
