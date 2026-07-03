//! Main Scan gameplay container component (Alpha-only minimal visor).

use crate::components::scan_board::ScanBoard;
use crate::components::scan_logic::{BoardState, GameStatus, Sector};
use crate::components::scan_overlay::ScanOverlay;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_status: Callback<Option<(String, String)>>,
}

#[function_component(ScanGame)]
pub fn scan_game(props: &Props) -> Html {
    let board = use_state(|| BoardState::new(Sector::Alpha));
    let flag_mode = use_state(|| false);
    let locale = use_context::<LocaleContext>().expect("locale context");

    let reset_game = {
        let board = board.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |_| {
            board.set(BoardState::new(Sector::Alpha));
            on_status.emit(Some((
                "Re-initialized. Ready to scan.".to_string(),
                "success".to_string(),
            )));
        })
    };

    let on_reveal = {
        let board = board.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |(r, c): (usize, usize)| {
            let mut new_board = (*board).clone();
            let old_status = new_board.status;

            new_board.reveal_cell(r, c);

            if old_status == GameStatus::NotStarted && new_board.status == GameStatus::Playing {
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
                    <button onclick={restart_click} class="btn-reset">{ locale.t("play_again") }</button>
                </div>
                <div class="beacons-counter">
                    <span class="hud-label">{"BEACONS:"}</span>
                    <span class="hud-value font-neon">{ remaining_beacons }</span>
                </div>
            </div>
        </div>
    }
}
