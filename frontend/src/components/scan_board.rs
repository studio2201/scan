//! Renders the Scan board grid.

use crate::components::scan_logic::BoardState;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub board: BoardState,
    pub flag_mode: bool,
    pub on_reveal: Callback<(usize, usize)>,
    pub on_flag: Callback<(usize, usize)>,
}

#[function_component(ScanBoard)]
pub fn scan_board(props: &Props) -> Html {
    let board = &props.board;
    let flag_mode = props.flag_mode;

    // Construct inline grid layout variables
    let grid_style = format!(
        "display: grid; grid-template-rows: repeat({}, 1fr); grid-template-columns: repeat({}, 1fr); gap: 4px; width: 100%; max-width: 900px; margin: 0 auto; aspect-ratio: {} / {};",
        board.rows, board.cols, board.cols, board.rows
    );

    html! {
        <div class="scan-board-wrapper">
            <div style={grid_style} class="scan-grid">
                { for (0..board.rows).flat_map(|r| {
                    let on_reveal = props.on_reveal.clone();
                    let on_flag = props.on_flag.clone();
                    (0..board.cols).map(move |c| {
                        let cell = board.grid[r][c];

                        let on_click = {
                            let on_reveal = on_reveal.clone();
                            let on_flag = on_flag.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                if flag_mode {
                                    on_flag.emit((r, c));
                                } else {
                                    on_reveal.emit((r, c));
                                }
                            })
                        };

                        let on_context_menu = {
                            let on_flag = on_flag.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                on_flag.emit((r, c));
                            })
                        };

                        let mut cell_class = vec!["scan-cell".to_string()];
                        let mut cell_content = html! {};

                        if cell.is_revealed {
                            cell_class.push("revealed".to_string());
                            if cell.is_mine {
                                cell_class.push("mine".to_string());
                                cell_content = html! { <span class="mine-icon">{"☤"}</span> };
                            } else if cell.adjacent_mines > 0 {
                                cell_class.push(format!("adjacent-{}", cell.adjacent_mines));
                                cell_content = html! { <span>{ cell.adjacent_mines }</span> };
                            }
                        } else if cell.is_flagged {
                            cell_class.push("flagged".to_string());
                            cell_content = html! { <span class="flag-icon">{"⚑"}</span> };
                        } else {
                            cell_class.push("covered".to_string());
                        }

                        html! {
                            <button
                                key={format!("{}-{}", r, c)}
                                class={classes!(cell_class)}
                                onclick={on_click}
                                oncontextmenu={on_context_menu}
                            >
                                { cell_content }
                            </button>
                        }
                    })
                }) }
            </div>
        </div>
    }
}
