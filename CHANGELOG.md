# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
