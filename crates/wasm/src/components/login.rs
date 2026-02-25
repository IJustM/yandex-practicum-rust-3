use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, logging::log, prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};

use crate::{
    components::notifications::{
        Notification, NotificationDesign, NotificationsState, get_notifications,
    },
    utils::get_url,
};

#[derive(Clone, Debug)]
enum Action {
    Login,
    Register,
}

#[derive(Serialize)]
struct UserForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct ServerError {
    pub message: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let (action, set_action) = signal::<Option<Action>>(None);

    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        match action.get() {
            Some(Action::Login) => spawn_local(async move {
                let res = Request::post(&get_url("/api/auth/login"))
                    .header("Content-Type", "application/json")
                    .json(&UserForm {
                        email: email.get(),
                        password: password.get(),
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if res.ok() {
                    let auth_response = res.json::<AuthResponse>().await;
                } else {
                    let error = res.json::<ServerError>().await.unwrap_or(ServerError {
                        message: res.text().await.unwrap_or("".to_string()),
                    });

                    let (_, set_notifications) = get_notifications();
                    set_notifications.update(|state| {
                        state.add(NotificationDesign::Error, &error.message);
                    });
                };
            }),
            Some(Action::Register) => {}
            _ => {}
        };
    };

    view! {
        <div class="w-full h-screen flex flex-col justify-center items-center gap-8">
            <h1 class="text-2xl">Login</h1>
            <form class="flex flex-col gap-4" on:submit=on_submit>
                <div class="flex flex-col gap-1">
                    <label for="email">email</label>
                    <input
                        required
                        type="text"
                        name="email"
                        class="border border-solid rounded-sm"
                        bind:value=(email, set_email)
                    />
                </div>
                <div class="flex flex-col gap-1">
                    <label for="password">password</label>
                    <input
                        required
                        type="password"
                        name="password"
                        bind:value=(password, set_password)
                        class="border border-solid rounded-sm"
                    />
                </div>
                <div class="flex gap-2 w-full">
                    <button
                        type="submit"
                        class="flex-1 rounded-sm bg-blue-500 text-white p-1 mt-4"
                        on:click=move |_| {
                            set_action.set(Some(Action::Login));
                        }
                    >
                        Sign in
                    </button>
                    <button
                        type="submit"
                        class="flex-1 rounded-sm bg-white text-gray-500 p-1 mt-4 border border-solid border-gray-500"
                        on:click=move |_| {
                            set_action.set(Some(Action::Register));
                        }
                    >
                        Sign up
                    </button>
                </div>
            </form>
        </div>
    }
}
