use leptos::prelude::*;
use leptos_router::{
    components::{ProtectedRoute, Redirect, Route, Router, Routes},
    path,
};

use crate::{
    api::is_jwt_token,
    components::{login::Login, notifications::Notifications, posts::Posts},
};

#[component]
pub fn App() -> impl IntoView {
    let not_found = || view! { <h1>"Not Found"</h1> };

    view! {
        <Router>
            <Routes fallback=not_found>
                <Route path=path!("/login") view=Login />

                <ProtectedRoute
                    path=path!("/posts")
                    redirect_path=|| "/login"
                    condition=|| { Some(is_jwt_token()) }
                    view=Posts
                />

                <Route path=path!("") view=|| view! { <Redirect path="/login" /> } />
                <Route path=path!("/*any") view=not_found />
            </Routes>
            <Notifications />
        </Router>
    }
}
