use crate::services::common_services::{get_accounts, Account};
use crate::services::error::ErrorString;
use leptos::ev::MouseEvent;
use leptos::*;

async fn get_accounts_result() -> Result<Vec<Account>, ErrorString> {
    let accounts = get_accounts().await;
    accounts
}

#[component]
pub fn GetAccountsExtension(// set_account_load: WriteSignal<(String, String)>
) -> impl IntoView {
    // let (accounts, set_accounts) = create_signal::<Result<Vec<Account>, ErrorString>>(Ok(vec![]));
    let (account_called, set_account_called) = create_signal(false);
    let (error, set_error) = create_signal("".to_string());

    let get_accounts_action = create_action(|()| async move { get_accounts_result().await });
    let pending_accounts = get_accounts_action.pending();
    let get_accounts_value = get_accounts_action.value();

    let onclick_button = move |e: MouseEvent, i| {
        gloo::console::log!(format!("Account number{}", i));
    };
    let get_accounts_click = move |e: MouseEvent| {
        get_accounts_action.dispatch(());
    };
    let accounts_html = move || {
        let accounts_option = get_accounts_value();
        let html_element = match accounts_option {
            Some(accounts_result) => match accounts_result {
                Ok(accounts_value) => {
                    if accounts_value.is_empty() {
                        view!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                    } else {
                        view! {
                            <div>
                                <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                                { move || accounts_value.iter().enumerate().map(|(i, account)| {
                                    view! {
                                        <button on:click=move |e| {onclick_button(e, i)}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                        </button>
                                    }
                                }).collect_view() }
                            </div>
                        }
                    }
                }
                Err(err) => view! {
                    <div>
                        <div class="error"> {"Error: "} {format!("{:?}", err.0)} </div>
                    </div>
                },
            },
            None => view! {
                <div>
                </div>
            },
        };
        html_element
    };

    view! {
        <>
            <div class="container">
                <button on:click=get_accounts_click> {"=> Select an Account for Signing"} </button>
                {if account_called() {
                    accounts_html()
                } else {
                    view! { <div></div> }
                }}
            </div>
        </>
    }
}
