# proptest-semver

> **Property Testing implementations for Semantic Versioning**

[![Crates.io](https://img.shields.io/crates/v/proptest-semver?style=flat-square)](https://crates.io/crates/proptest-semver)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/canardleteer/proptest-semver/testing.yml?branch=main&style=flat-square)](https://github.com/canardleteer/proptest-semver/actions/workflows/testing.yml?query=branch%3Amain)

This crate is generally meant for doing property testing on [Semantic Versioning
2.0.0](https://semver.org/). While working on
[sem-tool](https://github.com/canardleteer/sem-tool/), I found I needed to add
"way too much" for property testing, so broke out this crate instead.

While we can generally generate `String` to be parsed, we also take advantage of
the [semver](https://crates.io/crates/semver) crate, as that's a common choice
for Rust developers, and provide support for it. In particular, we support
generation of valid String for
[semver::VersionReq](https://docs.rs/semver/1.0.25/semver/struct.VersionReq.html)
(as well as sub-components).

## Notes

- I've never made a "pure testing" crate before, new territory of patterns for
  me.
- Usage is going to drive this to be a bit better. There's still some
  non-uniformity in this early release.

## Opinions

- I have found situations where the API method for creating `semver::VersionReq`
  is divergent from the String Parsing route, so I offer both routes of
  creation.
  - `arb_*_semver_*` patterned functions will take a "pure" `semver` creation
    approach.
    - This is probably faster, but offers less control.
  - All other functions, will usually generate `String`, and attempt to `parse()`
    a `semver` object out of it.
    - My use case drove this first, so it's got better handles for probability.
    - I expect most use cases take the `parse()` route.

## Usage

- Nothing much here yet, recommend looking at [tests/main.rs](tests/main.rs).
  - And eventual integration in `sem-tool`.

## TODO

- [ ] Pull all optional args up to the top.
- [ ] Avoid `prop_compose!` for public methods.
- [ ] `release-plz` pipeline
- [ ] binary to generate data
- [ ] remove misc default weights as magic numbers
- [ ] tease apart private api tests from public api tests
