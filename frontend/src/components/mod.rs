//! Yew components used by the Scan UI.
//!
//! Grouped by purpose:
//!
//! - [`header`] / [`footer`] — re-exports of the shared header and footer
//!   components that live in `shared-frontend`. Kept as local modules so
//!   consumers can `use crate::components::header::Header` without having
//!   to learn the upstream crate path.
//! - [`pin`] — PIN-gate login form.
//! - [`event_listener`] — RAII wrapper around `addEventListener` that
//!   removes the listener on drop.
//! - [`scan`] — the game itself: board, dpad, leaderboard, overlay,
//!   game logic, and the centralised state hook.
//! - [`scan_game`] — top-level game component that composes the scan
//!   sub-modules and renders the score / overlay layout.

pub mod footer;
pub mod header;
pub mod pin;
pub mod scan_board;
pub mod scan_game;
pub mod scan_leaderboard;
pub mod scan_logic;
pub mod scan_overlay;
