# Changelog

All notable changes to **cr8s** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased]

## [v0.1.2] – 2025-06-21

### Added
- New GitHub Actions CI workflow that enforces:
  - `cargo fmt --check`
  - `cargo clippy --release --all-targets --all-features -- -D warnings`
  - `cargo build --release`
  - `cargo test --release`
- Comprehensive integration test suite for all image commands (`blur`, `rotate`, `invert`, etc.)
  - Verifies pixel-level behavior (e.g. variance reduction after blur)
  - Validates CLI argument parsing and command execution paths
- New `run_mirage_command_suppress_output()` helper to reduce test log noise

### Changed
- Upgraded `LICENSE` from Boost to MIT for broader compatibility
- Updated `.gitignore` to better support Rust, editors, and JetBrains IDEs
- Replaced hardcoded `"Test_Image.PNG"` strings with a shared `TEST_IMAGE` constant
- Normalized file mode for test assets (`Test_Image.PNG`: 0755 → 0644)

### Removed
- Deleted `tests/foo.rs` and legacy `tests/err.log`

---

## [v0.1.1] – 2024-09-08

### Added
- Initial release of Mirage, a command-line image processing tool using `clap` and `image`.
- Support for subcommands:
  - `blur` – blur an image by percentage
  - `brighten` – brighten/darken an image
  - `crop` – crop to a rectangle
  - `rotate` – rotate by 90, 180, or 270 degrees
  - `invert` – invert pixel colors
  - `grayscale` – convert to grayscale
  - `fractal` – generate basic Mandelbrot render
  - `generate` – placeholder for future image generation
- CLI argument validation using `clap::value_parser!` and custom validators
- Macro-based wrapper (`imageop!`) to reduce repetition in subcommand dispatch
- Basic `tests/` folder with placeholder unit tests

### Changed
- Refactored to remove unused imports and `#[allow(unused)]`
- Added `README.md` with usage instructions
- Switched license from Boost 1.0 to MIT
- Introduced `rust.yml` GitHub workflow stub (later replaced)

### Notes
- No integration test coverage at this stage
- No CI enforcement or formatting/linting configured yet
