
# Changelog

## [Unreleased](https://github.com/SpinResearch/merkle.rs/compare/1.11.0...master)

## Changed
- Update to Rust 2018 ([@romac](https://github.com/romac))
- Update criterion to v0.3.3 ([@romac](https://github.com/romac))
- Update protobuf to v3.12.3 ([@romac](https://github.com/romac))

## [1.11.0](https://github.com/SpinResearch/merkle.rs/compare/1.10.0...1.11.0) - 2019-07-23

## Changed
- Update dependencies ([@romac](https://github.com/romac))

## Added
- Add `MerkleTree::gen_nth_proof` and `Proof::index` ([@afck](https://github.com/afck))

## [1.10.0](https://github.com/SpinResearch/merkle.rs/compare/1.9.0...1.10.0) - 2018-07-09

## Added
- Add `MerkleTree::gen_nth_proof` and `Proof::index` ([@afck](https://github.com/afck))

## [1.9.0](https://github.com/SpinResearch/merkle.rs/compare/1.8.0...1.9.0) - 2018-07-09

## Fixed
- Fix proof deserialization when deserializer does not own the slice ([@afck](https://github.com/afck))

## [1.8.0](https://github.com/SpinResearch/merkle.rs/compare/1.7.0...1.8.0) - 2018-07-07

## Changed
- Move `Proof::validate_lemma` to `Lemma::validate` ([@nvesely](https://github.com/nvesely))
- Automatically build Protobuf schemas using `protoc-rust` ([@dingxiangfei2009](https://github.com/dingxiangfei2009))
- Update `protobuf` to 2.0.2 ([@nvesely](https://github.com/nvesely))
- Update `ring` to `0.13.0` ([@kpcyrd](https://github.com/kpcyrd))

## [1.7.0](https://github.com/SpinResearch/merkle.rs/compare/1.6.0...1.7.0) - 2018-05-15

## Added
- Add optional `serde` support with the feature flag `serialization-serde`. ([@afck](https://github.com/afck))

## [1.6.0](https://github.com/SpinResearch/merkle.rs/compare/1.5.0...1.6.0) - 2018-05-15

### Changed
- Update to support rust-protobuf v1.6.0. ([@nvesely](https://github.com/nvesely))

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

