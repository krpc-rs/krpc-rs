# Changelog

## [0.3.0] - 2017/08/20
### Added
 - Basic stream handling (Warning: barely tested).
 - Added `CHANGELOG.md`
### Fixed
 - Instead of dumping entire TCP buffer to memory, only the first response is grabbed.

## [0.2.0] - 2017/08/20
### Added
 - Added `invoke!` macro, which simplifies calling RPCs, and reduces the client's dependencies protobuf.
### Changed
 - Moved from `quick-protobuf` to `rust-protobuf`.

## [0.1.1] - 2017/08/19
### Fixed
 - Fixed bug in which responses with no return value would wrongly return an error

## [0.1.0] - 2017/08/19
### Added
 - Added macros
### Changed
 - Renamed `Connection` to `Rpc`

## [0.0.2] - 2017/08/19
### Added
 - Added repository to `Cargo.toml`
 - Added license
### Changed
 - Renamed `krpc_rs` to `rpc`

## [0.0.1] - 2017/08/19
### Initial commit
