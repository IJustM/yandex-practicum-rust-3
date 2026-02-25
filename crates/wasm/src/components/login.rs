use gloo_net::http::Request;
use leptos::{control_flow::Show, ev::SubmitEvent, prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};

use crate::{
    api::{self},
    components::notifications::{NotificationDesign, get_signal_notifications},
    utils::get_url,
};

#[derive(Clone, Debug)]
enum Action {
    Login,
    Register,
}

#[derive(Serialize)]
struct UserLogin {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserRegister {
    email: String,
    password: String,
    username: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    pub access_token: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let (action, set_action) = signal::<Option<Action>>(None);
    let (is_registration, set_is_registration) = signal(false);

    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (username, set_username) = signal("".to_string());

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        match action.get() {
            Some(Action::Login) => spawn_local(async move {
                let req = api::add_json_data(
                    Request::post(&get_url("/api/auth/login")),
                    UserLogin {
                        email: email.get(),
                        password: password.get(),
                    },
                );
                if let Ok(res) = api::send::<AuthResponse>(req).await {
                    api::save_jwt_token(&res.access_token);
                }
            }),
            Some(Action::Register) => {
                spawn_local(async move {
                    let req = api::add_json_data(
                        Request::post(&get_url("/api/auth/register")),
                        UserRegister {
                            email: email.get(),
                            password: password.get(),
                            username: username.get(),
                        },
                    );
                    if let Ok(_) = api::send::<api::EmptyResponse>(req).await {
                        let (_, set_notifications) = get_signal_notifications();
                        set_notifications.update(|state| {
                            state.add(NotificationDesign::Success, "register success");
                        });
                        set_is_registration.set(false);
                        set_email.set("".to_string());
                        set_password.set("".to_string());
                        set_username.set("".to_string());
                    }
                });
            }
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
                <Show when=move || { is_registration.get() }>
                    <div class="flex flex-col gap-1">
                        <label for="username">username</label>
                        <input
                            required
                            type="username"
                            name="username"
                            bind:value=(username, set_username)
                            class="border border-solid rounded-sm"
                        />
                    </div>
                </Show>
                <div class="flex gap-2 w-full">
                    <Show when=move || { !is_registration.get() }>
                        <button
                            type="submit"
                            class="flex-1 rounded-sm p-1 mt-4 bg-blue-500 text-white"
                            on:click=move |_| {
                                set_action.set(Some(Action::Login));
                            }
                        >
                            Sign in
                        </button>
                        <button
                            type="submit"
                            class="flex-1 rounded-sm p-1 mt-4 bg-white text-gray-500 border border-solid border-gray-500"
                            on:click=move |e| {
                                e.prevent_default();
                                set_is_registration.set(true);
                            }
                        >
                            Sign up
                        </button>
                    </Show>

                    <Show when=move || { is_registration.get() }>
                        <button
                            type="submit"
                            class="flex-1 rounded-sm p-1 mt-4 bg-white text-gray-500 border border-solid border-gray-500"
                            on:click=move |e| {
                                e.prevent_default();
                                set_is_registration.set(false);
                            }
                        >
                            Back
                        </button>
                        <button
                            type="submit"
                            class="flex-1 rounded-sm p-1 mt-4 bg-green-500 text-white"
                            on:click=move |_| {
                                set_action.set(Some(Action::Register));
                            }
                        >
                            Sign up
                        </button>
                    </Show>
                </div>
            </form>
        </div>
    }
}
