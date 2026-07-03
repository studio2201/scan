//! Leaderboard display component for Scan.

use crate::api::ApiService;
use crate::components::scan_logic::Sector;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub sector: Sector,
    pub reload_trigger: usize,
}

#[function_component(ScanLeaderboard)]
pub fn scan_leaderboard(props: &Props) -> Html {
    let sector = props.sector;
    let reload_trigger = props.reload_trigger;
    let entries = use_state(Vec::new);
    let locale = use_context::<LocaleContext>().expect("locale context");

    {
        let entries = entries.clone();
        use_effect_with((sector, reload_trigger), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let category = sector.name();
                if let Ok(list) = ApiService::get_leaderboard(category).await {
                    entries.set(list);
                } else {
                    entries.set(Vec::new());
                }
            });
        });
    }

    html! {
        <div class="leaderboard-panel glassmorphic">
            <h3>{ format!("{} {}", sector.name(), locale.t("leaderboard")) }</h3>
            <div class="leaderboard-table-container">
                <table class="leaderboard-table">
                    <thead>
                        <tr>
                            <th>{ "RANK" }</th>
                            <th>{ "OPERATOR" }</th>
                            <th>{ locale.t("score") }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { if entries.is_empty() {
                            html! {
                                <tr>
                                    <td colspan="3" class="no-records">{ locale.t("no_scores") }</td>
                                </tr>
                            }
                        } else {
                            html! {
                                { for entries.iter().enumerate().map(|(idx, entry)| {
                                    html! {
                                        <tr>
                                            <td class="rank-col">{ format!("#{}", idx + 1) }</td>
                                            <td class="name-col">{ &entry.name }</td>
                                            <td class="score-col">{ format!("{:.1}s", entry.score as f64 / 10.0) }</td>
                                        </tr>
                                    }
                                }) }
                            }
                        } }
                    </tbody>
                </table>
            </div>
        </div>
    }
}
