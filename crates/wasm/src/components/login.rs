use gloo_net::http::Request;
use leptos::{
    control_flow::Show,
    ev::{MouseEvent, SubmitEvent},
    prelude::*,
    reactive::spawn_local,
};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use serde::{Deserialize, Serialize};

use crate::{
    api::{self},
    components::button::{Button, ButtonDesign},
    state::notifications::{NotificationDesign, get_signal_notifications},
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

    let navigate = use_navigate();

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        match action.get() {
            Some(Action::Login) => {
                let navigate = navigate.clone();
                spawn_local(async move {
                    let req = api::add_json_data(
                        Request::post(&get_url("/api/auth/login")),
                        UserLogin {
                            email: email.get(),
                            password: password.get(),
                        },
                    );
                    if let Ok(res) = api::send::<AuthResponse>(req).await {
                        api::save_jwt_token(&res.access_token);
                        navigate(
                            "/posts",
                            NavigateOptions {
                                replace: true,
                                ..Default::default()
                            },
                        );
                    }
                });
            }
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
                    if api::send::<api::EmptyResponse>(req).await.is_ok() {
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
            <form
                class="flex flex-col gap-4 border border-solid rounded-xl p-8 pt-5 border-gray-300"
                on:submit=on_submit
            >
                <h1 class="text-2xl">Login</h1>
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
                <div class="flex gap-3 w-full">
                    <Show when=move || { !is_registration.get() }>
                        <Button
                            design=ButtonDesign::Blue
                            on_click=move |_| {
                                set_action.set(Some(Action::Login));
                            }
                        >
                            Sign in
                        </Button>
                        <Button
                            design=ButtonDesign::Gray
                            on_click=move |e: MouseEvent| {
                                e.prevent_default();
                                set_is_registration.set(true);
                            }
                        >
                            Sign up
                        </Button>
                    </Show>

                    <Show when=move || { is_registration.get() }>
                        <Button
                            design=ButtonDesign::Gray
                            on_click=move |e: MouseEvent| {
                                e.prevent_default();
                                set_is_registration.set(false);
                            }
                        >
                            Back
                        </Button>
                        <Button
                            design=ButtonDesign::Green
                            on_click=move |_| {
                                set_action.set(Some(Action::Register));
                            }
                        >
                            Sign up
                        </Button>
                    </Show>
                </div>
            </form>
        </div>
    }
}
