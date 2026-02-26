use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{NavigateOptions, hooks::use_params, params::Params};
use uuid::Uuid;

use crate::{
    api::{add_authorization, add_json_data, get_url, send_request},
    components::button::{Button, ButtonDesign},
    domain::{CreatePostRequest, EmptyResponse, PostResponse, UpdatePostRequest},
    navigation::use_app_nav,
    state::notifications::{NotificationDesign, get_signal_notifications},
};

#[derive(Params, PartialEq, Clone)]
pub struct PostParams {
    id: Uuid,
}

#[component]
pub fn Post() -> impl IntoView {
    let (id, set_id) = signal::<Option<Uuid>>(None);

    let (title, set_title) = signal("".to_string());
    let (content, set_content) = signal("".to_string());

    let nav = use_app_nav();

    let params = use_params::<PostParams>();
    Effect::new(move |_| {
        match params.read().clone().map(|p| p.id) {
            Ok(id) => {
                set_id.set(Some(id));
                spawn_local(async move {
                    let req = Request::get(&get_url(&format!("/api/posts/{}", id.to_string())))
                        .build()
                        .unwrap();

                    if let Ok(post) = send_request::<PostResponse>(req).await {
                        set_title.set(post.title);
                        set_content.set(post.content);
                    }
                });
            }
            Err(_) => {
                set_id.set(None);
            }
        };
    });

    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-3xl">
                {if id.get().is_none() { "Create post" } else { "Update post" }}
            </h1>

            <div class="flex gap-2">
                <Show when=move || {
                    id.get().is_none()
                }>
                    {move || {
                        view! {
                            <Button
                                form="form-post".to_string()
                                design=ButtonDesign::Blue
                                on_click=move |_| {
                                    spawn_local(async move {
                                        let req = add_json_data(
                                            add_authorization(Request::post(&get_url("/api/posts"))),
                                            CreatePostRequest {
                                                title: title.get(),
                                                content: content.get(),
                                            },
                                        );
                                        if send_request::<PostResponse>(req).await.is_ok() {
                                            nav.to("/posts", NavigateOptions::default());
                                            let (_, set_notifications) = get_signal_notifications();
                                            set_notifications
                                                .update(|state| {
                                                    state.add(NotificationDesign::Success, "Post created");
                                                });
                                        }
                                    });
                                }
                            >
                                Create
                            </Button>
                        }
                    }}
                </Show>
                <Show when=move || {
                    id.get().is_some()
                }>
                    {move || {
                        view! {
                            <Button
                                form="form-post".to_string()
                                design=ButtonDesign::Blue
                                on_click=move |_| {
                                    spawn_local(async move {
                                        let req = add_json_data(
                                            add_authorization(
                                                Request::put(
                                                    &get_url(
                                                        &format!("/api/posts/{}", id.get().unwrap().to_string()),
                                                    ),
                                                ),
                                            ),
                                            UpdatePostRequest {
                                                title: title.get(),
                                                content: content.get(),
                                            },
                                        );
                                        if send_request::<PostResponse>(req).await.is_ok() {
                                            nav.to("/posts", NavigateOptions::default());
                                            let (_, set_notifications) = get_signal_notifications();
                                            set_notifications
                                                .update(|state| {
                                                    state.add(NotificationDesign::Success, "Post updated");
                                                });
                                        }
                                    });
                                }
                            >
                                Update
                            </Button>
                            <Button
                                form="form-post".to_string()
                                design=ButtonDesign::Red
                                on_click=move |_| {
                                    spawn_local(async move {
                                        let req = add_authorization(
                                                Request::delete(
                                                    &get_url(
                                                        &format!("/api/posts/{}", id.get().unwrap().to_string()),
                                                    ),
                                                ),
                                            )
                                            .build()
                                            .unwrap();
                                        if send_request::<EmptyResponse>(req).await.is_ok() {
                                            nav.to("/posts", NavigateOptions::default());
                                            let (_, set_notifications) = get_signal_notifications();
                                            set_notifications
                                                .update(|state| {
                                                    state.add(NotificationDesign::Success, "Post deleted");
                                                });
                                        }
                                    });
                                }
                            >
                                Remove
                            </Button>
                        }
                    }}
                </Show>
            </div>

            <form
                id="form-post"
                class="flex flex-col gap-4 max-w-[600px]"
                on:submit=|e| {
                    e.prevent_default();
                }
            >
                <div class="flex flex-col gap-1">
                    <label for="title">title</label>
                    <input
                        required
                        type="text"
                        name="title"
                        class="border border-solid rounded-sm"
                        bind:value=(title, set_title)
                    />
                </div>
                <div class="flex flex-col gap-1">
                    <label for="content">content</label>
                    <textarea
                        required
                        name="content"
                        bind:value=(content, set_content)
                        class="border border-solid rounded-sm"
                    />
                </div>
            </form>
        </div>
    }
}
