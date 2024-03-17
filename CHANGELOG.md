# Changelog
This file keeps track of notable changes in between versions.

This project uses [semantic versioning](https://semver.org). As such, we will mark which are breaking changes.

## [2.0.0] - 2024-03-17
### Removed
- `ErrorTraceFormatter` and `ErrorTraceColourFormatter` no longer support `Debug` (deemed unnecessary and resulting in easier fix below).

### Fixed
- `trace!()`- and `trace_coloured!()`-macros to now accept unsized error types.

## [1.2.1] - 2024-01-09
### Changed
- Errors input to `trace!()` or `trace_coloured!()` no longer need to be sized.


## [1.2.0] - 2024-01-09
### Changed
- The `trace!()`- and `trace_coloured!()`-macros to take the error by reference instead of ownership, as the temporary struct is temporary anyway.


## [1.1.1] - 2023-12-17
### Fixed
- Bad signature for the `trace!()`- and `trace_coloured!()`-macros.


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
