use crate::services::common_services::{get_accounts, Account};
use crate::services::error::ErrorString;
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn GetAccountsExtension(
    // set_account_load: WriteSignal<(String, String)>
) -> impl IntoView {
    let (accounts, set_accounts) = create_signal::<Result<Vec<Account>, ErrorString>>(Ok(vec![]));
    let (account_called, set_account_called) = create_signal(false);
    let onclick_button = move |e: MouseEvent, i| {
        gloo::console::log!(format!("Account number{}", i));
    };
    let get_accounts_click = move |e: MouseEvent| {};
    let accounts_html = move || match accounts() {
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
                } // <-- Call the closure to produce IntoView trait
            }
        }
        Err(err) => view! {
            <div>
                <div class="error"> {"Error: "} {format!("{:?}", err)} </div>
            </div>
        },
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
