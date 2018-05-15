
# Changelog

## [Unreleased](https://github.com/SpinResearch/merkle.rs/compare/1.6.0...master)

- Support serde with the feature flag `serialization-serde`.

> Nothing yet.

## [1.6.0](https://github.com/SpinResearch/merkle.rs/compare/1.5.0...1.6.0) - 2018-05-15

### Changed
- Update to support rust-protobuf v1.6.0 ([@nvesely](https://github.com/nvesely))

## [1.5.0](https://github.com/SpinResearch/merkle.rs/compare/1.4.1...1.5.0) - 2017-09-24

### Changed
- Update `ring` to v0.12.x. ([@romac](https://github.com/romac))

## [1.4.1](https://github.com/SpinResearch/merkle.rs/compare/1.4.0...1.4.1) - 2017-09-24

### Changed
- Take hashing algorithm into account for Eq/Ord impls. ([@romac](https://github.com/romac))

## [1.4.0](https://github.com/SpinResearch/merkle.rs/compare/1.3.0...1.4.0) - 2017-09-24

### Added
- Add missing PartialEq, Eq, PartialOrd, Ord and Hash impls ([@romac](https://github.com/romac))

## [1.3.0](https://github.com/SpinResearch/merkle.rs/compare/1.2.0...1.3.0) - 2017-08-03

### Changed
- Upgrade to *ring* 0.11 ([@romac](https://github.com/romac))

## [1.2.0](https://github.com/SpinResearch/merkle.rs/compare/1.1.0...1.2.0) - 2017-05-07

### Changed
- Use *ring* 0.9 to reduce build dependencies ([@briansmith](https://github.com/briansmith))

## [1.1.0](https://github.com/SpinResearch/merkle.rs/compare/1.0.0...1.1.0) - 2017-02-15

### Added
- Implement `Iterator` and `IntoIterator` for `MerkleTree` ([@romac](https://github.com/romac))

### Changed
- Avoid allocating memory on the heap in HashUtils, and made `combine_hashes` more efficient ([@briansmith](https://github.com/briansmith))
- Bump `ring` to `^0.6.0` and `protobuf` to `^1.2.0` ([@romac](https://github.com/romac))

## [1.0.0](https://github.com/SpinResearch/merkle.rs/releases/tag/1.0.0) - 2016-11-29

> Initial release

