use leptos::prelude::*;
use leptos_router::{
    components::{ProtectedRoute, Redirect, Route, Router, Routes},
    path,
};

use crate::{
    api::is_jwt_token,
    components::{login::Login, notifications::Notifications, post::Post, posts::Posts},
    navigation::use_init_app_nav,
};

#[component]
pub fn App() -> impl IntoView {
    let not_found = || view! { <h1>"Not Found"</h1> };

    view! {
        <Router>
            {move || {
                use_init_app_nav();

                view! {
                    <main>
                        <Routes fallback=not_found>
                            <Route path=path!("/login") view=Login />

                            <ProtectedRoute
                                path=path!("/posts/new")
                                redirect_path=|| "/login"
                                condition=|| { Some(is_jwt_token()) }
                                view=|| {
                                    view! { <Post /> }
                                }
                            />

                            <ProtectedRoute
                                path=path!("/posts/:id")
                                redirect_path=|| "/login"
                                condition=|| { Some(is_jwt_token()) }
                                view=|| {
                                    view! { <Post /> }
                                }
                            />

                            <Route path=path!("/posts") view=Posts />

                            <Route path=path!("") view=|| view! { <Redirect path="/posts" /> } />
                            <Route path=path!("/*any") view=not_found />
                        </Routes>
                        <Notifications />
                    </main>
                }
            }}
        </Router>
    }
}
