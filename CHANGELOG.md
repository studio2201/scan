# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.13] - 2026-07-19

### Fixed
- **TUI execution fix**: Resolved argument routing issue in the main entry point of the admin tool, enabling the "tui" parameter to launch the interactive dashboard successfully.


## [0.8.12] - 2026-07-19

### Changed
- **Uniform Rounded Icon**: Applied a rounded corner mask with white-filled borders to make all application icons perfectly uniform.


## [0.8.11] - 2026-07-19

### Changed
- **Simple Bright Icon**: Replaced application icon with a simple high-contrast 2-color flat-art neon cyan and purple vector illustration on a dark navy blue background.


## [0.8.10] - 2026-07-19

### Fixed
- **Warning fix**: Removed unused mutable keyword on server command spawn to prevent warning compilation failures in CI runners.


## [0.8.9] - 2026-07-19

### Changed
- **Release build bump**: Preparing new version release to trigger automated package container compilation on GHCR.


## [0.8.8] - 2026-07-19

### Changed
- **Slim Branding Banner**: Replaced the repository header banner with a slim, flat-art twilight landscape of Cheney, WA (home of the server) featuring rolling hills, Ponderosa pines, and a soaring neon eagle.


## [0.8.7] - 2026-07-19

### Changed
- **Containerized Admin Console integration**: Named the admin tool after the application (`scan`) and copied it to the container's system path `/usr/local/bin/scan`. The TUI can now be launched by simply executing `scan tui` (or `scan`) inside the container shell.
- **Documentation Modernization**: Rewrote `README.md` to remove CI details, format CLI commands as tables, and purge local development guides.


## [0.8.6] - 2026-07-19

### Changed
- **Containerized Admin Console integration**: Named the admin tool after the application (`scan`) and copied it to the container's system path `/usr/local/bin/scan`. The TUI can now be launched by simply executing `scan tui` (or `scan`) inside the container shell.
- **Documentation Modernization**: Rewrote `README.md` to remove CI details, format CLI commands as tables, and purge local development guides.


## [1.0.1] - 2026-07-19

### Changed
- Update README, clean file tree, and remove contributing/license files.


## [0.8.5] - 2026-07-19

### Changed
- **Standardized CLI & TUI command interface**: Aligned all admin commands and options with industry standard conventions. Added aliases for starting (`up`, `run`), stopping (`stop`, `down`), restarting (`restart`, `reload`), and diagnosing (`check`, `diagnose`) the application services.


## [0.8.4] - 2026-07-19

### Added
- **TUI & CLI Diagnostic Commands**: Added `doctor`, `start`, and `end`/`close` commands. Added the interactive system health check (doctor report) to the TUI panel menu.


## [0.8.3] - 2026-07-19

### Added
- **CLI Version Flag**: Added support for checking version details in the admin CLI using `version`, `-v`, or `--version` flags.


## [0.8.2] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [0.8.1] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [0.2.23] - 2026-07-19

### Changed
- **Rebrand to studio2201**: README, container labels, docker-compose, and Cargo
  metadata now reference `studio2201/scan`. CI badge URL and GHCR image name
  updated accordingly.
- **Replaced placeholder CHANGELOG**: prior entries were a copy-paste of the
  sister `snake` project's history and did not reflect Scan development. This
  release starts a fresh changelog with the rebrand entry; historical entries
  are intentionally omitted since they did not document Scan.
- **Manifest description**: `assets/manifest.json` description updated to match
  the README tagline ("Planetary hazard sector scanner").

## [0.1.0] - 2026-07-02

### Added
- Initial public release under the studio2201 organisation.
- Planetary hazard sector scanner web game built in Rust with Yew + WebAssembly
  frontend and Axum backend.
- Sector grid with progressively expanding scan area and flagging mechanic.
- SVG-based entity rendering and theming via the shared-frontend crate.
- Authenticated high-score leaderboard with cookie-based PIN.
