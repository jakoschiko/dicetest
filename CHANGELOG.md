# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Add function `dicetest::Fate::roll_distribution`. This allows to generate values directly from a `rand::distributions::Distribution`.
- Add optional feature flag `quickcheck_full`. This enables the integration of `quickcheck::Arbitrary`.
- Implement `rand_core::RngCore` for `dicetest::Fate`.
- Implement `quickcheck::Gen` for `dicetest::Fate`.
- Add function `dicetest::Fate::roll_arbitrary`. This allows to generate values for a type that implements `quickcheck::Arbitrary`.
- Add function `dicetest::dice::arbitrary`. This allows to create a `dicetest::Die` for a type that implements `quickcheck::Arbitrary`.
- Add struct `dicetest::hints::Section`.
- Add marco `dicetest::hint_section`. This makes it easier to work with hint indents.


[Unreleased]: https://github.com/jakoschiko/dicetest/compare/v0.2.0...HEAD
