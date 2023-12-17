# Changelog
This file keeps track of notable changes in between versions.

This project uses [semantic versioning](https://semver.org). As such, we will mark which are breaking changes.


## [1.1.0] - 2023-12-17
### Added
- The `trace!()`-macro, for quick, one-time error type creation with `ErrorTrace` support.
- `rustfmt.toml` for consistent formatting.

### Changed
- Coloured formatting no longer emits `error: ` before the error itself (to play nicely with things like [`log`](https://crates.io/crates/log)).

### Fixed
- Documentation for the `ErrorTraceColourFormatter` type.


## [1.0.0] - 2023-09-22
Initial release!

### Added
- `ErrorTrace` trait, with the `ErrorTraceFormatter` and `ErrorTraceColourFormatter`.
