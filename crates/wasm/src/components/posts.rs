use gloo_net::http::Request;
use leptos::{control_flow::Show, prelude::*};
use leptos_router::NavigateOptions;

use crate::{
    api::{clear_jwt_token, get_url, is_jwt_token, send_request},
    components::button::{Button, ButtonDesign},
    domain::PostListResponse,
    navigation::use_app_nav,
};

#[component]
pub fn Posts() -> impl IntoView {
    let nav: crate::navigation::AppNav = use_app_nav();

    let post_list = LocalResource::new(move || async move {
        let req = Request::get(&get_url("/api/posts")).build().unwrap();
        send_request::<PostListResponse>(req).await.unwrap()
    });

    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-3xl">Posts</h1>
            <div class="flex gap-2">
                <Show when=move || {
                    !is_jwt_token()
                }>
                    {move || {
                        view! {
                            <Button
                                design=ButtonDesign::Blue
                                on_click=move |_| {
                                    nav.to("/login", NavigateOptions::default());
                                }
                            >
                                Sign in
                            </Button>
                        }
                    }}
                </Show>
                <Show when=move || { is_jwt_token() }>
                    <Button
                        design=ButtonDesign::Gray
                        on_click=move |_| {
                            clear_jwt_token();
                            let _ = gloo_utils::window().location().reload();
                        }
                    >
                        Exit
                    </Button>

                    {move || {
                        view! {
                            <Button
                                design=ButtonDesign::Blue
                                on_click=move |_| {
                                    nav.to("/posts/new", NavigateOptions::default());
                                }
                            >
                                Create new
                            </Button>
                        }
                    }}
                </Show>
            </div>

            {move || {
                post_list
                    .read()
                    .clone()
                    .map(|post_list| {
                        view! {
                            <table class="border-collapse border">
                                <thead>
                                    <tr>
                                        {vec!["Id", "Title", "Content", "Created at", "Author id"]
                                            .into_iter()
                                            .map(|text| {
                                                view! {
                                                    <th class="border bg-gray-100 py-1 px-2">{text}</th>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {post_list
                                        .posts
                                        .into_iter()
                                        .map(|p| {
                                            view! {
                                                <tr
                                                    class="cursor-pointer hover:bg-blue-200"
                                                    on:click=move |_| {
                                                        nav.to(
                                                            format!("/posts/{}", p.id.to_string()),
                                                            NavigateOptions::default(),
                                                        );
                                                    }
                                                >
                                                    {vec![
                                                        p.id.to_string(),
                                                        p.title,
                                                        p.content,
                                                        p.created_at.to_string(),
                                                        p.author_id.to_string(),
                                                    ]
                                                        .into_iter()
                                                        .map(|value| {
                                                            view! { <td class="border py-1 px-3">{value}</td> }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        }
                    })
            }}
        </div>
    }
}
