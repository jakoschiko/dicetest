# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Add function `dicetest::dice::todo`. This generator is for prototyping and always panics when created.
- Add functions `dicetest::dice::split_{u8,u16,u32,u64,u128,usize}`. These generators are similar to `dicetest::dice::split_{u8,u16,u32,u64,u128,usize}_n`, but use a const parameter for the number of parts.
- Add function `dicetest::dice::split_vec_n`. This generators is similar to `dicetest::dice::split_vec`, but uses a non-const parameter for the number of parts.
- Add function `dicetest::dice::split_limit`. This generator splits a `dicetest::Limit` into random parts.
- Add function `dicetest::dice::split_limit_n`. This generators is similar to `dicetest::dice::split_limit`, but uses a non-const parameter for the number of parts.
- Add support for regression tests
  - Add function `dicetest::Dicetest::regression` for adding a regression test.
  - Add function `dicetest::Dicetest::regressions_enabled` for enabling/disabling regression tests.
  - Add environment variable `DICETEST_REGRESSIONS_ENABLED` for enabling/disabling regression tests.
  - Add struct `dicetest::runner::repeatedly::Regression` and field `dicetest::runner::repeatedly::Config::regressions`.
- Add trait `dicetest::Dice` that provides a `dicetest::Die` for a type.
- Add derive macro for `dicetest::Dice`.
- Add feature flag `derive` for enabling the derive macro.
- Add function `dicetest::die` that provides a `dicetest::Die` based on `dicetest::Dice`.
- Add `dicetest::Dice` to `dicetest::prelude`.
- Add `dicetest::die` to `dicetest::prelude`.

### Fixed
- Fix unintentional panic in `dicetest::dice::weighted_*` if sum of weights is zero.

### Changed
- Rename functions `dicetest::dice::terms_of_{u8,u16,u32,u64,u128,usize}` to `dicetest::dice::split_{u8,u16,u32,u64,u128,usize}_n`.
- Change signature of `dicetest::dice::split_vec`. Instead of returning a pair with two parts, it now has a type parameter `const N: usize` and returns an array with `N` parts.
- Rename feature flag `rand_full` to `rand`.
- Upgrade dependency rand_core to 0.6 and rand to 0.8.
- Set MSRV to 1.87.0

### Removed
- Remove feature flag `quickcheck_full` and the integration of `quickcheck::Gen` and `quickcheck::Arbitrary` due to missing functionality in quickcheck 1.0.
- Remove function `dicetest::Fate::roll_distribution`. Use `dicetest::dice::from_distribution` instead.

## [0.3.1] - 2022-02-27

### Fixed
- Fix unintentional panic in `dicetest::dice::one_of_die_once` and `dicetest::dice::one_of_die`.

## [0.3.0] - 2021-09-10

### Added
- Add function `dicetest::dice::shuffle_slice`. This allows to shuffle a slice in-place.
- Add function `dicetest::dice::array`. This allows to generate arrays using const generics.

### Removed
- Remove function `dicetest::dice::array_1`, ..., `dicetest::dice::array_32`. Use `dicetest::dice::array` instead.

### Changed
- Rename `dicetest::dice::size` to `dicetest::dice::length` and `dicetest::dice::SizeRange` to `dicetest::dice::LengthRange`. This expresses better their intention and prevents confusion with `dicetest::dice::usize` and `dicetest::dice::isize`.
- Reorganize functions `dicetest::dice::zip*`, `dicetest::dice::one_of*` and `dicetest::dice::weighted_one_of*`. For each group of functions with same functionality but different arity a struct is added that bundles these functions as methods. E.g. instead of `dice::one_of_2(1, 2)` you can write now `dice::one_of().two(1, 2)`.

## [0.2.1] - 2020-12-05

### Added
- Add function `dicetest::Fate::roll_distribution`. This allows to generate values directly from a `rand::distributions::Distribution`.
- Add optional feature flag `quickcheck_full`. This enables the integration of `quickcheck::Arbitrary`.
- Implement `rand_core::RngCore` for `dicetest::Fate`.
- Implement `quickcheck::Gen` for `dicetest::Fate`.
- Add function `dicetest::Fate::roll_arbitrary`. This allows to generate values for a type that implements `quickcheck::Arbitrary`.
- Add function `dicetest::dice::arbitrary`. This allows to create a `dicetest::Die` for a type that implements `quickcheck::Arbitrary`.
- Add struct `dicetest::hints::Section`.
- Add marco `dicetest::hint_section`. This makes it easier to work with hint indents.

[Unreleased]: https://github.com/jakoschiko/dicetest/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/jakoschiko/dicetest/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jakoschiko/dicetest/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/jakoschiko/dicetest/compare/v0.2.0...v0.2.1
