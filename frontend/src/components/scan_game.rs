//! Main Scan gameplay container component.

use crate::api::ApiService;
use crate::components::scan_board::ScanBoard;
use crate::components::scan_leaderboard::ScanLeaderboard;
use crate::components::scan_logic::{BoardState, GameStatus, Sector};
use crate::components::scan_overlay::ScanOverlay;
use crate::i18n::LocaleContext;
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_status: Callback<Option<(String, String)>>,
}

#[function_component(ScanGame)]
pub fn scan_game(props: &Props) -> Html {
    let sector = use_state(|| Sector::Alpha);
    let board = use_state(|| BoardState::new(Sector::Alpha));
    let flag_mode = use_state(|| false);
    let elapsed = use_state(|| 0u32);
    let interval_handle = use_mut_ref(|| None::<Interval>);
    let reload_trigger = use_state(|| 0usize);
    let locale = use_context::<LocaleContext>().expect("locale context");

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
        let sector = sector.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |s: Sector| {
            stop_timer();
            elapsed.set(0);
            board.set(BoardState::new(s));
            sector.set(s);
            on_status.emit(Some((
                format!("Visor initialized. Ready to scan {} sector.", s.name()),
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

    let on_submit_score = {
        let elapsed = elapsed.clone();
        let sector = sector.clone();
        let reload_trigger = reload_trigger.clone();
        let on_status = props.on_status.clone();
        let sector_val = *sector;
        let reset_game = reset_game.clone();

        Callback::from(move |name: String| {
            let elapsed_val = *elapsed;
            let category = sector_val.name().to_string();
            let reload_trigger = reload_trigger.clone();
            let on_status = on_status.clone();
            let reset_game = reset_game.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if ApiService::submit_score(&name, elapsed_val, &category)
                    .await
                    .is_ok()
                {
                    reload_trigger.set(*reload_trigger + 1);
                    on_status.emit(Some((
                        "Scan record catalogued.".to_string(),
                        "success".to_string(),
                    )));
                    reset_game.emit(sector_val);
                } else {
                    on_status.emit(Some((
                        "Failed to upload score.".to_string(),
                        "error".to_string(),
                    )));
                }
            });
        })
    };

    let sector_alpha = {
        let reset_game = reset_game.clone();
        Callback::from(move |_| reset_game.emit(Sector::Alpha))
    };
    let sector_beta = {
        let reset_game = reset_game.clone();
        Callback::from(move |_| reset_game.emit(Sector::Beta))
    };
    let sector_gamma = {
        let reset_game = reset_game.clone();
        Callback::from(move |_| reset_game.emit(Sector::Gamma))
    };
    let restart_click = {
        let reset_game = reset_game.clone();
        let sector = sector.clone();
        Callback::from(move |_| reset_game.emit(*sector))
    };
    let on_restart_overlay = {
        let reset_game = reset_game.clone();
        let sector = sector.clone();
        Callback::from(move |()| reset_game.emit(*sector))
    };
    let toggle_flag_mode = {
        let flag_mode = flag_mode.clone();
        Callback::from(move |_| flag_mode.set(!*flag_mode))
    };

    let remaining_beacons = board.mines as isize - board.count_flagged() as isize;

    html! {
        <div class="game-container">
            <div class="game-main-panel">
                <div class="hud-bar glassmorphic">
                    <div class="hud-metric">
                        <span class="hud-label">{ format!("{}:", locale.t("sector").to_uppercase()) }</span>
                        <span class="hud-value font-neon">{ sector.name().to_uppercase() }</span>
                    </div>
                    <div class="hud-metric">
                        <span class="hud-label">{ "BEACONS:" }</span>
                        <span class="hud-value font-neon">{ remaining_beacons }</span>
                    </div>
                    <div class="hud-metric">
                        <span class="hud-label">{ format!("{}:", locale.t("score").to_uppercase()) }</span>
                        <span class="hud-value font-neon">{ format!("{:.1}s", *elapsed as f64 / 10.0) }</span>
                    </div>
                </div>

                <div class="control-row">
                    <div class="sector-buttons">
                        <button onclick={sector_alpha} class={if *sector == Sector::Alpha { "active" } else { "" }}>{"ALPHA"}</button>
                        <button onclick={sector_beta} class={if *sector == Sector::Beta { "active" } else { "" }}>{"BETA"}</button>
                        <button onclick={sector_gamma} class={if *sector == Sector::Gamma { "active" } else { "" }}>{"GAMMA"}</button>
                    </div>
                    <div class="mode-toggles">
                        <button onclick={toggle_flag_mode} class={if *flag_mode { "active" } else { "" }}>
                            { if *flag_mode { "⚑ BEACON" } else { "⛏ REVEAL" } }
                        </button>
                        <button onclick={restart_click} class="btn-reset">{ locale.t("play_again") }</button>
                    </div>
                </div>

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
                                elapsed_tenths={*elapsed}
                                on_restart={on_restart_overlay}
                                on_submit={on_submit_score}
                            />
                        }
                    } else {
                        html! {}
                    } }
                </div>
            </div>

            <div class="game-side-panel">
                <ScanLeaderboard sector={*sector} reload_trigger={*reload_trigger} />
            </div>
        </div>
    }
}
