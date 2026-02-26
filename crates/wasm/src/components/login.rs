use gloo_net::http::Request;
use leptos::{
    control_flow::Show,
    ev::{MouseEvent, SubmitEvent},
    prelude::*,
    reactive::spawn_local,
};
use leptos_router::NavigateOptions;

use crate::{
    api::{add_json_data, get_url, save_jwt_token, send_request},
    components::button::{Button, ButtonDesign},
    domain::{AuthResponse, EmptyResponse, LoginRequest, RegisterRequest},
    navigation::use_app_nav,
    state::notifications::{NotificationDesign, get_signal_notifications},
};

#[derive(Clone, Debug)]
enum Action {
    Login,
    Register,
}

#[component]
pub fn Login() -> impl IntoView {
    let (action, set_action) = signal::<Option<Action>>(None);
    let (is_registration, set_is_registration) = signal(false);

    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (username, set_username) = signal("".to_string());

    let nav = use_app_nav();

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        match action.get() {
            Some(Action::Login) => {
                spawn_local(async move {
                    let req = add_json_data(
                        Request::post(&get_url("/api/auth/login")),
                        LoginRequest {
                            email: email.get(),
                            password: password.get(),
                        },
                    );

                    if let Ok(res) = send_request::<AuthResponse>(req).await {
                        save_jwt_token(&res.access_token);
                        nav.to(
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
                    let req = add_json_data(
                        Request::post(&get_url("/api/auth/register")),
                        RegisterRequest {
                            email: email.get(),
                            password: password.get(),
                            username: username.get(),
                        },
                    );
                    if send_request::<EmptyResponse>(req).await.is_ok() {
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
                <h1 class="text-3xl">Login</h1>
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
                            is_flex_one=true
                            on_click=move |_| {
                                set_action.set(Some(Action::Login));
                            }
                        >
                            Sign in
                        </Button>
                        <Button
                            design=ButtonDesign::Gray
                            is_flex_one=true
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
                            is_flex_one=true
                            on_click=move |e: MouseEvent| {
                                e.prevent_default();
                                set_is_registration.set(false);
                            }
                        >
                            Back
                        </Button>
                        <Button
                            design=ButtonDesign::Green
                            is_flex_one=true
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
