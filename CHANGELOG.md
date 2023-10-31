# Change log

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.10] - 2023-10-31

### Changed

- When include and/or link paths are specified explicitly, these paths must be provided to all
  compiler instances, both when discovering compiler paths, and when executing `bindgen`.

Thank you very much, *wiiznokes*.

## [0.2.8] - 2023-09-26

### Added

- Documentation on how to install `libsensors`.
- Hint to install `libsensors` and its C header files when these cannot be found by `build.rs`.

Thank you very much, *Carter*.

### Changed

- Updated dependencies.

## [0.2.7] - 2023-08-09

### Changed

- Updated dependencies.

## [0.2.6] - 2023-06-07

### Changed

- Updated build script to better integrate with `cargo`.

## [0.2.5] - 2023-04-18

### Changed

- Updated dependencies.

## [0.2.4] - 2022-11-22

### Changed

- Updated dependencies.

## [0.2.3] - 2022-11-20

### Changed

- Updated dependencies.

## [0.2.2] - 2022-09-03

### Changed

- Updated dependencies.

## [0.2.1] - 2021-10-25

### Changed

- Added a *Documentation-only* build mode to allow building documentation even
  if `libsensors` and its headers are unavailable.
  This allows building on `docs.rs`.

## [0.2.0] - 2021-10-25

### Changed

- Switched to Rust edition 2021.
  > **This is a breaking change**.

## [0.1.0] - 2021-10-13

### Added

- Initial release.
