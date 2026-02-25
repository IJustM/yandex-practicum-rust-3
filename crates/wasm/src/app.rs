use leptos::prelude::*;

use crate::components::{login::Login, notifications::Notifications};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Login />
        <Notifications />
    }
}
