use crate::components::schelling_game::profile_validation::storage::get_period_fn::get_period_fn;
use leptos::prelude::*;

#[component]
pub fn GetPeriod(profile_user_account: String) -> impl IntoView {
    let period = get_period_fn(profile_user_account);
    let period_value = move || match period() {
        Some(value) => format!("Period name: {:?}", value),
        None => format!(""),
    };
    view! {
        <div>
            <code>{move || period_value()}</code>
        </div>
    }
}
