# Changelog
This file keeps track of notable changes in between versions.

This project uses [semantic versioning](https://semver.org). As such, we will mark which are breaking changes with **(BREAKING)**.

## [4.0.0] - 2025-07-01
### Added
- Some `crates.io`-preferred Cargo.toml sections.

### Changed
- Renamed all UK `coloured` to US `colored`. **(BREAKING)**
- Renamed `Trace` to `FrozenTrace` for better naming. **(BREAKING)**
- Renamed `trace!()` to `toplevel!()` for disambiguation with e.g. [`log`](https://docs.rs/log/latest/log/)'s `trace!()`-macro. **(BREAKING)**
- Renamed `trace_coloured!()` to `toplevel_colored!()`. **(BREAKING)**

### Fixed
- Documentation not including items behind non-default features.
  - To properly show them with the feature gates, use the nightly `rustdoc` version.
- Tests not running.

## [3.3.1] - 2025-06-20
### Changed
- Lowered minimum versions in `Cargo.toml` to actually the minimum supported versions.

## [3.3.0] - 2024-11-05
### Added
- `Trace::from_source()` from when the trace needs an additional message on top of the given error.

## [3.2.1] - 2024-10-24
### Fixed
- The `trace!()`- and `trace_coloured!()`-macros no longer have an unnecessary `'static`-bound on input error types.

## [3.2.0] - 2024-10-23
### Added
- The optional `serde`-feature, which implements `Deserialize` and `Serialize` for `Trace`.

## [3.1.0] - 2024-10-10
### Added
- The `Trace` helper type, which can be used to serialize an error and all its sources and then store in another type.
    - This is useful in cases when there are lifetimes on an error but it should be passed outside of those lifetimes. See the documentation of `Trace` for an example.


## [3.0.0] - 2024-10-08
This update sees a license change. From now on, the project is available under the [Apache 2.0](./LICENSE) license **(BREAKING)**.


## [2.0.0] - 2024-07-22
### Removed
- `ErrorTraceFormatter` and `ErrorTraceColourFormatter` no longer support `Debug` (deemed unnecessary and resulting in easier fix below) **(BREAKING)**.

### Fixed
- `trace!()`- and `trace_coloured!()`-macros to now accept unsized error types.


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
