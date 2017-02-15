
# Changelog

## [Unreleased](https://github.com/SpinResearch/merkle.rs/compare/1.1.0...master)

> Nothing yet.

## [1.1.0](https://github.com/SpinResearch/merkle.rs/compare/1.0.0...1.1.0) - 2017-02-15

## Added
- Implement `Iterator` and `IntoIterator` for `MerkleTree` ([@romac](https://github.com/romac))

## Changed
- Avoid allocating memory on the heap in HashUtils, and made `combine_hashes` more efficient ([@briansmith](https://github.com/briansmith))
- Bump `ring` to `^0.6.0` and `protobuf` to `^1.2.0` ([@romac](https://github.com/romac))


## [1.0.0](https://github.com/SpinResearch/merkle.rs/releases/tag/1.0.0) - 2016-11-29

> Initial release

