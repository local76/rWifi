# Changelog

All notable changes to this project will be documented in this file.

## [3.1.0] - 2026-06-08

### Changed
- Renamed project back from `scout-tui` to `scout` (crate name: `scout`, binary name: `scout`).
- Split monolithic `src/wlan.rs` (1276 lines) into modular `src/wlan/` files (all under 500 lines).
- Split monolithic `src/main.rs` (2025 lines) into `src/app/`, `src/ui/`, and `src/win32.rs` submodules (all under 500 lines).
- Fixed the selection highlight bug where single mouse clicks on TUI list items or buttons incorrectly triggered full-line selection and clipboard copy.
- Introduced a drag threshold check to prevent mouse clicks from starting text selection unless a drag occurs.

## [3.0.1] - 2026-06-06
### Added
- Added author and maintainer metadata for packaging.

## [3.0.0] - 2026-06-06
### Changed
- Renamed organization to `local76`.
- Renamed executable from `rtem` to `scout`.
- Reorganized directory structure to group packaging files inside `dist/packages/`.