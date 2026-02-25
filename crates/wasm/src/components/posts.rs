use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    api::{self},
    utils::get_url,
};

#[component]
pub fn Posts() -> impl IntoView {
    let post_list = LocalResource::new(move || async move {
        let req = Request::get(&get_url("/api/posts")).build().unwrap();
        api::send::<PostList>(req).await.unwrap()
    });

    view! {
        <h1 class="text-2xl">Posts</h1>
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
                                            <tr class="cursor-pointer hover:bg-blue-200">
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
    }
}

#[derive(Deserialize, Clone, Serialize)]
struct PostList {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Clone, Serialize)]
struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
