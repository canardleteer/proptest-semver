//! SPDX-License-Identifier: Apache-2.0
//! Copyright 2025 canardleteer
//!
//! Licensed under the Apache License, Version 2.0 (the "License"); you may not
//! use this file except in compliance with the License. You may obtain a copy
//! of the License at
//!
//! <http://www.apache.org/licenses/LICENSE-2.0>
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
//! WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
//! License for the specific language governing permissions and limitations
//! under the License.
//!
//! This crate generally provides out of the box `proptest` implementations for
//! doing Property Testing of Semantic Version values with the `semver` crate.
//!
//! The Regex from the spec is available here: <https://semver.org/>, and where
//! most of these come from.
use proptest::prelude::*;
use proptest_derive::Arbitrary;
use semver::{Version, VersionReq};
use std::fmt;

/// Regex for Semantic Version 2.0.0, directly from the spec, with 2 changes:
///
/// * ASCII Only Restriction
/// * No prepended `^` or trailing `$`, since [proptest!] uses this with the
///   [regex_generate](https://github.com/CryptArchy/regex_generate) crate.
pub const SEMVER_REGEX: &str = r"(?-u:(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)";

/// Regex for the "Major.Minor.Patch" component of SemVer.
///
/// NOTE(canardleteer): The `semver` crate, for `Version``, does not support
///                     anything over u64::MAX.
pub const MAJOR_MINOR_PATCH_REGEX: &str = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)";

/// Regex for any single Major, Minor, Patch component.
///
/// NOTE(canardleteer): The `semver` crate, for `Version``, does not support
///                     anything over u64::MAX.
pub const ANY_MAJOR_MINOR_PATH_COMPONENT: &str = r"^(0|[1-9]\d*)";

/// Regex to build a Pre-Release string, sometimes, with the prefix `-`.
pub const SOMETIMES_PRERELEASE_REGEX: &str = r"(?-u:(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?)";

/// Regex to build a Build Metadata string, sometimes, with the prefix `+`.
pub const SOMETIMES_BUILD_METADATA_REGEX: &str =
    r"(?-u:(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)";

/// Regex to build a Pre-Release string, always, without the `-`.
pub const ALWAYS_PRERELEASE_REGEX: &str = r"(?-u:(?:((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*)))";

/// Regex to build a Build Metadata string, always, without the prefix `+`.
pub const ALWAYS_BUILD_METADATA_REGEX: &str = r"(?-u:(?:([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*)))";

/// Describes the a hard limit set in [semver::VersionReq] (`32`). This limit
/// isn't exposed, so we re-encode it here, and then we test that it's the right
/// version.
///
/// If it ever changes, we will fail that test.
pub const MAX_COMPARATORS_IN_VERSION_REQ_STRING: usize = 32;

// These should be used as choices when Option is None. Currently it is
// effectively hard coded in places.
const DEFAULT_PROBABILITY_OF_PRE_RELEASE: f64 = 0.5;
const DEFAULT_PROBABILITY_OF_BUILD_METADATA: f64 = 0.5;

prop_compose! {
    /// Arbitrary Semantic Versioning 2.0.0 String.
    ///
    /// WARNING(canardleteer): Most common implementations, do not support
    /// arbitrary length MAJOR.MINOR.PATCH` components, and prefer to treat them
    /// as standard integers.
    ///
    /// While not "to the spec," is generally reasonable and common practice.
    /// Consider using [arb_version] for something more commonly used in
    /// practice.
    pub fn arb_semver()(s in SEMVER_REGEX) -> String {
        s
    }
}

prop_compose! {
    /// Arbitrary Pre-Release String (no `-` prefix)
    pub fn arb_pre_release_string()(pr in ALWAYS_PRERELEASE_REGEX) -> String {
        pr
    }
}

prop_compose! {
    pub fn arb_semver_prerelease()(pr in arb_pre_release_string()) -> semver::Prerelease {
        semver::Prerelease::new(&pr).unwrap()
    }
}

prop_compose! {
    /// Arbitrary Optional Pre-Release String (no `-` prefix)
    ///
    /// * `probability_of_some` - Follows [proptest::option::Probability] rules.
    pub fn arb_option_pre_release_string(probability_of_some: f64)(pr in prop::option::weighted(probability_of_some, ALWAYS_PRERELEASE_REGEX)) -> Option<String> {
        pr
    }
}

prop_compose! {
    pub fn arb_option_semver_prerelease(probability_of_some: f64)(pr in arb_option_pre_release_string(probability_of_some)) -> Option<semver::Prerelease> {
        pr.map(|pr| semver::Prerelease::new(&pr).unwrap())
    }
}

prop_compose! {
    /// Arbitrary Build Metadata String (no `+` prefix)
    pub fn arb_build_metadata_string()(md in ALWAYS_BUILD_METADATA_REGEX) -> String {
        md
    }
}

prop_compose! {
    pub fn arb_semver_build_metadata()(bm in arb_build_metadata_string()) -> semver::BuildMetadata {
        semver::BuildMetadata::new(&bm).unwrap()
    }
}

prop_compose! {
    /// Arbitrary Optional Build Metadata String (no `+` prefix)
    ///
    /// * `probability_of_some` - Follows [proptest::option::Probability] rules.
    pub fn arb_option_build_metadata_string(probability_of_some: f64)(md in prop::option::weighted(probability_of_some,ALWAYS_BUILD_METADATA_REGEX)) -> Option<String> {
        md
    }
}

prop_compose! {
    pub fn arb_option_semver_build_metadata(probability_of_some: f64)(bm in arb_option_build_metadata_string(probability_of_some)) -> Option<semver::BuildMetadata> {
        bm.map(|bm| semver::BuildMetadata::new(&bm).unwrap())
    }
}

/// Implement a generic strategy around `semver:Op`.
///
/// Since "Wildcard is often useless for testing `VersionReq`, by default it's weight is very weak.
///
/// I'm sure someone will eventually want all kinds broken out with weights.
///
/// * `default_weight` - The "weight" of picking every other option, except [semver::Op::Wildcard].
/// * `wildcard_weight` - The "weight" of picking [semver::Op::Wildcard], against the sum of all "default_weight" types.
pub fn arb_semver_op(
    default_weight: Option<u32>,
    wildcard_weight: Option<u32>,
) -> BoxedStrategy<semver::Op> {
    let default_weight = default_weight.unwrap_or(5);
    let wildcard_weight = wildcard_weight.unwrap_or(1);

    prop_oneof! [
        default_weight => Just(semver::Op::Exact),
        default_weight => Just(semver::Op::Greater),
        default_weight => Just(semver::Op::GreaterEq),
        default_weight => Just(semver::Op::Less),
        default_weight => Just(semver::Op::LessEq),
        default_weight => Just(semver::Op::Tilde),
        default_weight => Just(semver::Op::Caret),
        wildcard_weight => Just(semver::Op::Wildcard),
    ]
    .boxed()
}

prop_compose! {
    pub fn arb_semver_comparator()(op in arb_semver_op(None, None), major in any::<u64>(), minor in any::<Option<u64>>(), patch in any::<Option<u64>>(), pre in arb_semver_prerelease()) -> semver::Comparator {
        semver::Comparator{
            op, major, minor, patch, pre
        }
    }
}

prop_compose! {
    pub fn arb_vec_semver_comparator(max_len: usize)(vec in prop::collection::vec(arb_semver_comparator(), 1..max_len)) -> Vec<semver::Comparator> {
        vec
    }
}

prop_compose! {
    pub fn arb_semver_version_req(max_len: usize)(comparators in arb_vec_semver_comparator(max_len)) -> VersionReq {
        VersionReq {comparators}
    }
}

prop_compose! {
    pub fn arb_optional_semver_version_req(probability_of_some: f64, max_comparators: usize)(comparators in prop::option::weighted(probability_of_some, arb_vec_semver_comparator(max_comparators))) -> Option<VersionReq> {
        comparators.map(|comparators| VersionReq{comparators})
    }
}

prop_compose! {
    /// Creates a [semver::Version] from a [String] based on explicit weighting of Pre-Release & Build Metadata probability.
    ///
    /// * `probability_of_pre_release` - Follows [proptest::option::Probability] rules.
    /// * `probability_of_build_metadata` - Follows [proptest::option::Probability] rules.
    pub fn arb_version_weighted(probability_of_pre_release: f64, probability_of_build_metadata: f64)(major in any::<u64>().prop_map(|v| v.to_string()), minor in any::<u64>().prop_map(|v| v.to_string()), patch in any::<u64>().prop_map(|v| v.to_string()), pr in arb_option_pre_release_string(probability_of_pre_release), bm in arb_option_build_metadata_string(probability_of_build_metadata)) -> Version {
        let fmt_string = match (pr, bm) {
            (None, None) => {format!("{major}.{minor}.{patch}")},
            (None, Some(bm)) => {format!("{major}.{minor}.{patch}+{bm}")},
            (Some(pr), None) => {format!("{major}.{minor}.{patch}-{pr}")},
            (Some(pr), Some(bm)) => {format!("{major}.{minor}.{patch}-{pr}+{bm}")},
        };
        Version::parse(&fmt_string).unwrap()
    }
}

prop_compose! {
    /// Creates a valid [semver::Version] via `String`.
    pub fn arb_version()(v in arb_version_weighted(DEFAULT_PROBABILITY_OF_PRE_RELEASE, DEFAULT_PROBABILITY_OF_BUILD_METADATA)) -> Version {
        v
    }
}

prop_compose! {
    /// Creates a valid [semver::Version] via the struct itself.
    pub fn arb_semver_version_weighted(probability_of_pre_release: f64, probability_of_build_metadata: f64)(major in any::<u64>(), minor in any::<u64>(), patch in any::<u64>(), pre in arb_option_semver_prerelease(probability_of_pre_release), build in arb_option_semver_build_metadata(probability_of_build_metadata)) -> Version {
        let pre = pre.unwrap_or(semver::Prerelease::new("").unwrap());
        let build = build.unwrap_or(semver::BuildMetadata::new("").unwrap());

        Version{major, minor, patch, pre, build}
    }
}

prop_compose! {
    /// Creates a valid [semver::Version] via the struct itself.
    pub fn arb_semver_version()(major in any::<u64>(), minor in any::<u64>(), patch in any::<u64>(), pre in arb_option_semver_prerelease(DEFAULT_PROBABILITY_OF_PRE_RELEASE), build in arb_option_semver_build_metadata(DEFAULT_PROBABILITY_OF_BUILD_METADATA)) -> Version {
        let pre = pre.unwrap_or(semver::Prerelease::new("").unwrap());
        let build = build.unwrap_or(semver::BuildMetadata::new("").unwrap());

        Version{major, minor, patch, pre, build}
    }
}

prop_compose! {
    /// Creates a list of [semver::Version], with some specified length.
    ///
    /// * `max_len` - Maximum length of Vec to generate.
    pub fn arb_vec_versions(max_len: usize)(vec in prop::collection::vec(arb_version(), 1..max_len)) -> Vec<Version> {
        vec
    }
}

prop_compose! {
    /// Creates a list of [semver::Version], with some specified length.
    ///
    /// * `max_len` - Maximum length of Vec to generate.
    pub fn arb_vec_semver_versions(max_len: usize)(vec in prop::collection::vec(arb_semver_version(), 1..max_len)) -> Vec<Version> {
        vec
    }
}

prop_compose! {
    /// Creates a `String` that can be parsed as a valid [semver::Comparator]
    /// when building a [semver::VersionReq].
    pub fn arb_comparator_string()(c in arb_full_comparator(None, None, None).prop_map(|c| c.to_string())) -> String {
        c
    }
}

prop_compose! {
    /// Creates a `Vec<String>` that can be parsed as a valid
    /// [semver::Comparator]'s when building a [semver::VersionReq].
    pub fn arb_vec_comparator_string(max_len: usize)(vec in prop::collection::vec(arb_full_comparator(None, None, None).prop_map(|c| c.to_string()), 1..max_len)) -> Vec<String> {
        vec
    }
}

prop_compose! {
    /// Creates a [semver::VersionReq] of some maximum number of `Comparator`s.
    ///
    /// NOTE(canardleteer): As currently implemented, this `VersionReq` will be
    ///                     completely ridiculous, and is unlikely to match
    ///                     anything.
    ///
    /// * `max_comparators` - Should always be less than or equal to
    ///   [MAX_COMPARATORS_IN_VERSION_REQ_STRING].
    pub fn arb_version_req(max_comparators: usize)(comparators in arb_full_comparator_vec(max_comparators, None, None)) -> VersionReq {
        VersionReq::parse(&comparators.to_string()).unwrap()
    }
}

prop_compose! {
    /// Creates an Option<[semver::VersionReq]> of some maximum number of
    /// `Comparator`s.
    ///
    /// See notes in [arb_version_req].
    ///
    /// * `probability_of_some` - Follows [proptest::option::Probability] rules.
    /// * `max_comparators` - Should always be less than or equal to
    ///   [MAX_COMPARATORS_IN_VERSION_REQ_STRING].
    pub fn arb_optional_version_req(probability_of_some: f64, max_comparators: usize)(comparators in prop::option::weighted(probability_of_some, arb_vec_comparator_string(max_comparators))) -> Option<VersionReq> {
        comparators.map(|comparators| VersionReq::parse(&comparators.join(",")).unwrap())
    }
}

/// ComparatorVec captures the odd case that your [semver::VersionReq], can
/// basically only be one of two things.
///
/// * A single wildcard `*`
/// * A list of Comparators, with no single wildcard as any Comparator (but it can include
///   `<op>MAJOR.*` or `<op>MAJOR.MINOR.*`)
///
/// This can easily be made a [String], or can be sent to a [VersionReq::parse] if
/// joined with ",".
#[derive(Clone, Debug)]
pub enum ComparatorVec {
    List(Vec<FullComparator>),
    Wildcard,
}

prop_compose! {
    /// Provides a valid Vec<[semver::Comparator]> that can be fed to
    /// [semver:VersionReq::parse] after being run through `.join(',')`
    ///
    /// * `max_comparators` - The maximum number of comparators to return in this
    ///   list.
    pub fn arb_comparator_list(max_comparators: usize)(list in prop::collection::vec(arb_full_comparator(None, None, None), max_comparators)) -> Vec<FullComparator> {
        list
    }
}

/// Provides a weighted list of [ComparatorVec], for use in [arb_version_req],
/// but you may want to use it for your own means.
///
/// See the [proptest::prop_oneof!] macro for more information about weight
/// args.
///
/// * `max_comparators` - The maximum number of comparators to return in the
///   [ComparatorVec::List] type.
/// * `weight_of_wildcard` - (default: 1) Weight for this to be a
///   [ComparatorVec::Wildcard]. The "Wildcard" type is somewhat useless for
///   testing real [semver::VersionReq], but provided for completeness.
/// * `weight_of_comparator_list` - (default: 14) Weight for this to be a
///   [ComparatorVec::List].
pub fn arb_full_comparator_vec(
    max_comparators: usize,
    weight_of_wildcard: Option<u32>,
    weight_of_comparator_list: Option<u32>,
) -> impl Strategy<Value = ComparatorVec> {
    let weight_of_wildcard = weight_of_wildcard.unwrap_or(1);
    let weight_of_comparator_list = weight_of_comparator_list.unwrap_or(14);

    prop_oneof![
        weight_of_wildcard => Just(ComparatorVec::Wildcard),
        weight_of_comparator_list => arb_comparator_list(max_comparators).prop_map(ComparatorVec::List),
    ]
    .boxed()
}

impl fmt::Display for ComparatorVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ComparatorVec::List(full_comparators) => &full_comparators
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(","),
            ComparatorVec::Wildcard => "*",
        };
        write!(f, "{s}")
    }
}

/// Describes various types of [semver::Comparator] as [String].
#[derive(Clone, Debug)]
pub enum FullComparator {
    /// The common case of `<operator>MAJOR.MINOR.PATCH<-Pre-Release><+Build Metadata>`
    Plain(ComparatorOp, u64, u64, u64, Option<String>, Option<String>),

    /// The less common case of `<operator>MAJOR.*`
    WildcardMinor(ComparatorOp, u64),

    /// The less common case of `<operator>MAJOR.MINOR.*`
    WildcardPatch(ComparatorOp, u64, u64),

    // While a pure non-operational Wildcard can be a valid Comparator, it
    // becomes invalid when present with other Comparators by implementation
    // of the [semver::VersionReq].
    //
    // It is never chosen by the Strategy implementations.
    Wildcard,
}

/// Provides some kind of [semver::Comparator] "thing", sidestepping the
/// [FullComparator::Wildcard] case.
///
/// See the [proptest::prop_oneof!] macro for more information about weight
/// args.
///
/// * `weight_of_plain` - (default: 7) Weight for this to be a
///   [FullComparator::Plain], this is the most complex case.
/// * `weight_of_wildcard_minor` - (default: 1) Weight for this to be a
///   [FullComparator::WildcardMinor].
/// * `weight_of_wildcard_patch` - (default: 1) Weight for this to be a
///   [FullComparator::WildcardPatch].
pub fn arb_full_comparator(
    weight_of_plain: Option<u32>,
    weight_of_wildcard_minor: Option<u32>,
    weight_of_wildcard_patch: Option<u32>,
) -> impl Strategy<Value = FullComparator> {
    // We weight heavily on non-wildcard cases by default, since they "test less".

    let weight_of_wildcard_minor = weight_of_wildcard_minor.unwrap_or(1);
    let weight_of_wildcard_patch = weight_of_wildcard_patch.unwrap_or(1);
    let weight_of_plain = weight_of_plain.unwrap_or(7);

    prop_oneof![
        weight_of_wildcard_minor => (
            any::<ComparatorOp>(),
            any::<u64>(),
        ).prop_map(|(op, major)| FullComparator::WildcardMinor(op, major)),
        weight_of_wildcard_patch => (
            any::<ComparatorOp>(),
            any::<u64>(),
            any::<u64>(),
        ).prop_map(|(op, major, minor)| FullComparator::WildcardPatch(op, major, minor)),
        weight_of_plain => (
            any::<ComparatorOp>(),
            any::<u64>(),
            any::<u64>(),
            any::<u64>(),
            arb_option_pre_release_string(0.8),
            arb_option_build_metadata_string(0.8)
        )
            .prop_map(|(op, major, minor, patch, pr, bm)| FullComparator::Plain(op, major, minor, patch, pr, bm)),
    ]
    .boxed()
}

impl fmt::Display for FullComparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FullComparator::Plain(op, major, minor, patch, pr, bm) => {
                write!(f, "{op}{major}.{minor}.{patch}")?;

                if let Some(pr) = pr {
                    write!(f, "-{pr}")?
                }

                if let Some(bm) = bm {
                    write!(f, "+{bm}")?
                }

                Ok(())
            }
            FullComparator::WildcardMinor(op, major) => {
                write!(f, "{op}{major}.*.*")
            }
            FullComparator::WildcardPatch(op, major, patch) => {
                write!(f, "{op}{major}.{patch}.*")
            }
            FullComparator::Wildcard => {
                write!(f, "*")
            }
        }
    }
}

/// `ComparatorOp` is just a re-implementation of [semver::Op].
///
/// This could be removed, and a `Strategy` for [semver::Op] probably could be
/// defined with some kind of reasonable `fmt::Display` ().
#[derive(Arbitrary, Clone, Debug)]
pub enum ComparatorOp {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
    Caret,
}

impl fmt::Display for ComparatorOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ComparatorOp::Exact => "=",
            ComparatorOp::Greater => ">",
            ComparatorOp::GreaterEq => ">=",
            ComparatorOp::Less => "<",
            ComparatorOp::LessEq => "<=",
            ComparatorOp::Tilde => "~",
            ComparatorOp::Caret => "^",
        };
        write!(f, "{s}")
    }
}

// Enc Property Test components.
///////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        ///////////////////////////////////////////////////////////////////////
        // MAX_COMPARATORS_IN_VERSION_REQ_STRING Testing
        #[test]
        fn expected_max_comparators_test_high(a in arb_version_req(MAX_COMPARATORS_IN_VERSION_REQ_STRING)){
            let _ = a;
        }
        // MAX_COMPARATORS_IN_VERSION_REQ_STRING: Test B
        #[test]
        #[should_panic]
        fn expected_max_comparators_test_above_line(a in arb_version_req(MAX_COMPARATORS_IN_VERSION_REQ_STRING + 1)){
            let _ = a;
        }
        //
        ///////////////////////////////////////////////////////////////////////
    }
}
