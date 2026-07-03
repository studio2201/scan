//! Game status overlays (victory / defeat) for Scan.

use crate::components::scan_logic::GameStatus;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub status: GameStatus,
    pub on_restart: Callback<()>,
}

#[function_component(ScanOverlay)]
pub fn scan_overlay(props: &Props) -> Html {
    let locale = use_context::<LocaleContext>().expect("locale context");

    let restart_click = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_| on_restart.emit(()))
    };

    match props.status {
        GameStatus::Won => {
            html! {
                <div class="game-overlay victory glassmorphic">
                    <h2 class="outcome-title secured">{ "SECTOR SECURED" }</h2>
                    <p class="stat-line">{"Geothermal hazard mapping successful."}</p>
                    <button class="btn-restart" onclick={restart_click}>
                        { locale.t("play_again") }
                    </button>
                </div>
            }
        }
        GameStatus::Lost => {
            html! {
                <div class="game-overlay defeat glassmorphic">
                    <h2 class="outcome-title compromised">{ locale.t("game_over") }</h2>
                    <p class="stat-line">{"Sector scanning aborted due to thermal hazard."}</p>
                    <button class="btn-restart" onclick={restart_click}>
                        { locale.t("play_again") }
                    </button>
                </div>
            }
        }
        _ => html! {},
    }
}
