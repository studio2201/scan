//! Main Scan gameplay container component (Alpha-only minimal visor with timer).

use crate::components::scan_board::ScanBoard;
use crate::components::scan_logic::{BoardState, GameStatus, Sector};
use crate::components::scan_overlay::ScanOverlay;
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_status: Callback<Option<(String, String)>>,
}

#[function_component(ScanGame)]
pub fn scan_game(props: &Props) -> Html {
    let board = use_state(|| BoardState::new(Sector::Alpha));
    let flag_mode = use_state(|| false);
    let elapsed = use_state(|| 0u32);
    let interval_handle = use_mut_ref(|| None::<Interval>);

    // Stop timer on component drop
    {
        let interval_handle = interval_handle.clone();
        use_effect_with((), move |_| {
            move || {
                *interval_handle.borrow_mut() = None;
            }
        });
    }

    let start_timer = {
        let elapsed = elapsed.clone();
        let interval_handle = interval_handle.clone();
        let board = board.clone();
        move || {
            let elapsed = elapsed.clone();
            let board = board.clone();
            let interval = Interval::new(100, move || {
                if board.status == GameStatus::Playing {
                    elapsed.set(*elapsed + 1);
                }
            });
            *interval_handle.borrow_mut() = Some(interval);
        }
    };

    let stop_timer = {
        let interval_handle = interval_handle.clone();
        move || {
            *interval_handle.borrow_mut() = None;
        }
    };

    let reset_game = {
        let board = board.clone();
        let elapsed = elapsed.clone();
        let stop_timer = stop_timer.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |_| {
            stop_timer();
            elapsed.set(0);
            board.set(BoardState::new(Sector::Alpha));
            on_status.emit(Some((
                "Re-initialized. Ready to scan.".to_string(),
                "success".to_string(),
            )));
        })
    };

    let on_reveal = {
        let board = board.clone();
        let start_timer = start_timer.clone();
        let stop_timer = stop_timer.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let mut new_board = (*board).clone();
            let old_status = new_board.status;

            new_board.reveal_cell(r, c);

            if old_status == GameStatus::NotStarted && new_board.status == GameStatus::Playing {
                start_timer();
                on_status.emit(Some((
                    "Scanning sector... Detonation hazards present.".to_string(),
                    "success".to_string(),
                )));
            } else if new_board.status == GameStatus::Playing {
                on_status.emit(Some((
                    format!("Grid sector ({}, {}) scanned: Clear.", c + 1, r + 1),
                    "success".to_string(),
                )));
            }

            if new_board.status == GameStatus::Won || new_board.status == GameStatus::Lost {
                stop_timer();
                if new_board.status == GameStatus::Won {
                    on_status.emit(Some((
                        "Sector secured successfully.".to_string(),
                        "success".to_string(),
                    )));
                } else {
                    on_status.emit(Some((
                        "Danger! Detonation hazard encountered.".to_string(),
                        "error".to_string(),
                    )));
                }
            }
            board.set(new_board);
        })
    };

    let on_flag = {
        let board = board.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let mut new_board = (*board).clone();
            let was_flagged = new_board.grid[r][c].is_flagged;
            new_board.toggle_flag(r, c);
            let is_flagged = new_board.grid[r][c].is_flagged;
            if was_flagged != is_flagged {
                if is_flagged {
                    on_status.emit(Some((
                        format!("Beacon deployed at ({}, {}).", c + 1, r + 1),
                        "success".to_string(),
                    )));
                } else {
                    on_status.emit(Some((
                        format!("Beacon retrieved from ({}, {}).", c + 1, r + 1),
                        "success".to_string(),
                    )));
                }
            }
            board.set(new_board);
        })
    };

    let restart_click = {
        let reset_game = reset_game.clone();
        Callback::from(move |_| reset_game.emit(()))
    };

    let toggle_flag_mode = {
        let flag_mode = flag_mode.clone();
        Callback::from(move |_| flag_mode.set(!*flag_mode))
    };

    let remaining_beacons = board.mines as isize - board.count_flagged() as isize;

    html! {
        <div class="game-container">
            <div class="board-frame">
                <ScanBoard
                    board={(*board).clone()}
                    flag_mode={*flag_mode}
                    on_reveal={on_reveal}
                    on_flag={on_flag}
                />

                { if board.status == GameStatus::Won || board.status == GameStatus::Lost {
                    html! {
                        <ScanOverlay
                            status={board.status}
                            on_restart={reset_game.clone()}
                        />
                    }
                } else {
                    html! {}
                } }
            </div>

            <div class="control-row-minimal">
                <div class="mode-toggles">
                    <button onclick={toggle_flag_mode} class={if *flag_mode { "active" } else { "" }}>
                        { if *flag_mode { "⚑ BEACON" } else { "⛏ REVEAL" } }
                    </button>
                    { if board.status == GameStatus::Playing {
                        html! {
                            <button onclick={restart_click} class="btn-reset">{"RESTART"}</button>
                        }
                    } else if board.status == GameStatus::NotStarted {
                        html! {
                            <button class="btn-reset-guide" disabled=true>{"CLICK GRID TO START"}</button>
                        }
                    } else {
                        html! {}
                    } }
                </div>
                <div class="stats-counter">
                    <div class="beacons-counter">
                        <span class="hud-label">{"BEACONS:"}</span>
                        <span class="hud-value font-neon">{ remaining_beacons }</span>
                    </div>
                    <div class="timer-counter">
                        <span class="hud-label">{"TIME:"}</span>
                        <span class="hud-value font-neon">{ format!("{:.1}s", *elapsed as f64 / 10.0) }</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
