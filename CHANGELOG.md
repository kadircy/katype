# Changelog

## [Unreleased]
### Added
- Implemented support for all types which supports `Display` to colorize with function.

### Changed
- Removed unnecessary `.to_string` calls from logging.

## [0.2.2] - 2025-02-15
### Added
- Added support for changing "Be Ready" (ready text) to custom one.
- Added support for setting a timeout duration to test.
- Added option to CLI for generating code with custom words.
- Added an function to generate code from `&str` (words split by comma).
- Added option to CLI for printing results in JSON format.

### Changed
- The way of calculating consistency is changed for better overview. Now it continues even if an error occures.
- Changed seperator from comma to whitespace for generating codes.

## [0.2.1] - 2025-02-07
### Added
- Added option to CLI for changing language to different one from English.
- Implemented base64 codes for tests. Now you can share your tests with codes.
- Added comment lines for more readable code.
- Added an description to CLI.
- `cargo clippy` standarts implemented for better code.

### Changed
- The whole workspace refactored to be more performant.

## [0.1.1] - 2025-02-05
### Added
- Writed comments to the code workspace for better understanding and DX.
- Version information for CLI.

### Changed
- The format of 'acc' changed from `{}` to `{}%` for better UX.

### Fixed
- The warnings of `Rust compiler` are fixed. (Removed unused imports)

## [0.1.0] - 2025-02-04

### Added

- The whole project
