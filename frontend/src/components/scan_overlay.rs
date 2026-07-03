//! Game status overlays (victory / defeat) for Scan.

use crate::components::scan_logic::GameStatus;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub status: GameStatus,
    pub elapsed_tenths: u32,
    pub on_restart: Callback<()>,
    pub on_submit: Callback<String>,
}

#[function_component(ScanOverlay)]
pub fn scan_overlay(props: &Props) -> Html {
    let name_ref = use_node_ref();
    let name_val = use_state(String::new);
    let locale = use_context::<LocaleContext>().expect("locale context");

    let on_input = {
        let name_val = name_val.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val: String = input
                    .value()
                    .to_uppercase()
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .take(3)
                    .collect();
                input.set_value(&val);
                name_val.set(val);
            }
        })
    };

    let on_submit_click = {
        let on_submit = props.on_submit.clone();
        let name_val = name_val.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if !name_val.is_empty() {
                on_submit.emit((*name_val).clone());
            }
        })
    };

    let restart_click = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_| on_restart.emit(()))
    };

    match props.status {
        GameStatus::Won => {
            html! {
                <div class="game-overlay victory glassmorphic">
                    <h2 class="outcome-title secured">{ "SECTOR SECURED" }</h2>
                    <p class="stat-line">
                        { format!("{}: ", locale.t("final_score")) }
                        <span class="highlight">{ format!("{:.1}s", props.elapsed_tenths as f64 / 10.0) }</span>
                    </p>
                    <div class="score-submission-form">
                        <label for="initials-input">{ locale.t("enter_name") }</label>
                        <div class="input-row">
                            <input
                                id="initials-input"
                                ref={name_ref}
                                type="text"
                                maxlength="3"
                                placeholder="AAA"
                                oninput={on_input}
                                autocomplete="off"
                                autofocus=true
                            />
                            <button
                                class="btn-submit"
                                onclick={on_submit_click}
                                disabled={name_val.len() != 3}
                            >
                                { locale.t("submit_score") }
                            </button>
                        </div>
                    </div>
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
