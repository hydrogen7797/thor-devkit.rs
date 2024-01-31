use crate::components::accounts::accounts_handling::add_account::AddAccount;
use crate::components::home::Home;
use crate::components::markdown::markdown_component::MarkdownHtmlView;
use leptos::*;
use leptos_router::*;

#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/add-account" view=AddAccount/>
                <Route path="/markdown" view=MarkdownHtmlView/>
            </Routes>
        </Router>
    }
}
