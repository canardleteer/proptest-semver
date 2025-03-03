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
use proptest_semver::*;
use semver::{Comparator, Version, VersionReq};

proptest! {
    #![proptest_config(ProptestConfig {
        // Setting both fork and timeout is redundant since timeout implies
        // fork, but both are shown for clarity.
        fork: true,
        // timeout: 10000,
        cases: 256 * 1,
        .. ProptestConfig::default()
    })]

        #[test]
        fn test_semver(s in arb_semver()) {
            println!("{s}");
            prop_assert!(s.is_ascii());
            match Version::parse(&s) {
                Ok(_) => {},
                Err(e) => {
                    // Since Error is opaque, we can only do string matching
                    // here.
                    //
                    // This is a known (and reasonable) weakness in the `semver`
                    // crate.
                    if !e.to_string().contains("version number exceeds u64::MAX") {
                        panic!("unknown error from semver crate")
                    }
                }
            }
        }

        #[test]
        fn test_pre_release(pr in arb_option_pre_release_string(0.5)) {
            match pr {
                Some(pr) => {
                    prop_assert!(pr.is_ascii());
                    semver::Prerelease::new(&pr).unwrap();
                },
                None => {}
            }
        }

        #[test]
        fn test_semver_pre_release(pr in arb_option_semver_prerelease(0.5)) {
            let _ = pr;
            // println!("arb_semver_pre_release: {:?}", pr);
        }

        #[test]
        fn test_build_metadata(bm in arb_option_build_metadata_string(0.5)) {
            match bm {
                Some(bm) => {
                    prop_assert!(bm.is_ascii());
                    semver::BuildMetadata::new(&bm).unwrap();
                },
                None => {}
            }
        }

        #[test]
        fn test_semver_build_metadata(bm in arb_option_semver_build_metadata(0.5)) {
            let _ = bm;
            // println!("arb_semver_build_metadata: {:?}", bm);
        }

        #[test]
        fn test_version(a in arb_version()) {
            let _ = a;
        }

        #[test]
        fn test_semver_version(a in arb_version()) {
            let _ = a;
        }

        #[test]
        fn test_vec_versions(a in arb_vec_versions(128)) {
            prop_assert!(a.len() <= 128);
            let _ = a;
        }

        #[test]
        fn test_vec_semver_versions(a in arb_vec_semver_versions(128)) {
            prop_assert!(a.len() <= 128);
            let _ = a;
        }

        #[test]
        fn test_comparator(a in arb_comparator_string()) {
            Comparator::parse(&a).unwrap();
        }

        #[test]
        fn test_semver_comparator(a in arb_semver_comparator()) {
            let _ = a;
            // println!("semver::Comparator: {:?}", a);
        }

        #[test]
        fn test_vec_comparator(a in arb_vec_comparator_string(MAX_COMPARATORS_IN_VERSION_REQ_STRING)) {
            prop_assert!(a.len() <= MAX_COMPARATORS_IN_VERSION_REQ_STRING);
            for c in a {
                Comparator::parse(&c).unwrap();
            }
        }

        #[test]
        fn test_vec_semver_comparator(a in arb_vec_semver_comparator(MAX_COMPARATORS_IN_VERSION_REQ_STRING)) {
            prop_assert!(a.len() <= MAX_COMPARATORS_IN_VERSION_REQ_STRING);
            // println!("Vec<semver::Comparator>: {:?}", a);
        }

        #[test]
        fn test_arb_version_req(a in arb_version_req(MAX_COMPARATORS_IN_VERSION_REQ_STRING), v in arb_version()) {
            a.matches(&v);
        }

        #[test]
        fn test_arb_semver_version_req(a in arb_semver_version_req(MAX_COMPARATORS_IN_VERSION_REQ_STRING), v in arb_version()) {
            // println!("semver::VersionReq: {:?}", a);
            a.matches(&v);
        }

        #[test]
        fn test_arb_optional_version_req(a in arb_optional_version_req(0.5, MAX_COMPARATORS_IN_VERSION_REQ_STRING), v in arb_version()) {
            match a {
                Some(r) => {
                    let _ = r.matches(&v);
                },
                None => {},
            }
        }

        #[test]
        fn test_arb_optional_semver_version_req(a in arb_optional_semver_version_req(0.5, MAX_COMPARATORS_IN_VERSION_REQ_STRING), v in arb_version()) {
            match a {
                Some(r) => {
                    println!("semver::VersionReq: {:?}", r);
                    let _ = r.matches(&v);
                },
                None => {},
            }
        }

        #[test]
        fn test_arb_comparator_list(a in arb_comparator_list(MAX_COMPARATORS_IN_VERSION_REQ_STRING), v in arb_version()) {
            prop_assert!(a.len() <= MAX_COMPARATORS_IN_VERSION_REQ_STRING);

            let s = a.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
            let r = VersionReq::parse(&s).unwrap();
            let _ = r.matches(&v);
        }

        #[test]
        fn test_arb_full_comparator(a in arb_full_comparator(None, None, None), v in arb_version()) {
            VersionReq::parse(&a.to_string()).unwrap().matches(&v);
        }
}
