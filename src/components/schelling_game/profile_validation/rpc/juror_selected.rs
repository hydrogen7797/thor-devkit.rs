use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(profile_user_account: String, set_juror_selected: WriteSignal<Option<bool>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: bool = client
        .request(
            "profilevalidation_selectedjuror",
            rpc_params![profile_user_account],
        )
        .await
        .unwrap();
    set_juror_selected(Some(result));
}

#[component]
pub fn JurorSelected(profile_user_account: String) -> impl IntoView {
    let (juror_selected, set_juror_selected) = create_signal::<Option<bool>>(None);

    let action = create_action(
        |(profile_user_account, set_juror_selected): &(String, WriteSignal<Option<bool>>)| {
            let profile_user_account = profile_user_account.clone();
            let set_juror_selected = set_juror_selected.clone();
            async move { load_data(profile_user_account, set_juror_selected).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((profile_user_account.clone(), set_juror_selected));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if juror_selected().is_some() {
                    view! {
                        <div>
                            {"Juror Selected "}
                            <span id="juror-selected">{move || juror_selected()}</span>
                        </div>
                    }
                } else {
                    view! {
                        <div>
                            <span id="juror-selected">
                                <Icon
                                    icon=icondata::ImSpinner6
                                    style="color: green"
                                    class="inline-block"
                                />
                            </span>
                        </div>
                    }
                }
            }}

        </div>
    }
}
