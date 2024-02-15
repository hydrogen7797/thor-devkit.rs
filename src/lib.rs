mod components;
mod constants;
mod js_extension_binding;
mod router;
mod services;
use leptos::*;
use router::RouterApp;

#[component]
pub fn App() -> impl IntoView {
    view! { <RouterApp/> }
}
