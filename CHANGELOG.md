# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2025-04-19

### New Codecs

- Add a `bincode` feature to encode with `bincode v2` without serde and native `bincode::Encode` and `bincode::Decode` derive macros (thanks to @zakstucke)

## [0.3.0] - 2025-01-09

### Breaking Changes

- Updated rkyv to 0.8 (thanks to @thestarmaker)

## [0.2.0] - 2024-08-23

### Breaking Changes

- Trait `IsBinary` has been removed.
- Instead, the traits `HybridEncoder` and `HybridDecoder` now provide the methods `is_binary_encoder` and
  `is_binary_decoder` respectively.

## [0.1.2] - 2024-07-08

### New Codecs

- Added `MiniserdeCodec`
- Added `SerdeLite` wrapper for serde based codecs
- Added `JsonSerdeWasmCodec`

## [0.1.1] - 2024-07-07

### New Codec

- Added `RkyvCodec`

## [0.1.0] - 2024-07-07

Initial release.
