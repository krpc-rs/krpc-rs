# Changelog

##[0.3.1] - 2017/08/21
### Fixed
 - Responses over 127 bytes in length would be truncated, as the increment in the size of the bitshift in the varint decoding was `1`, not `7`
 - Removed comment that indicated that streams didn't work at all

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
