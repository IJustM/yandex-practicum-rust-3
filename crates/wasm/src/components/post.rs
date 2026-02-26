use leptos::prelude::*;
use leptos_router::NavigateOptions;
use uuid::Uuid;

use crate::{
    components::button::{Button, ButtonDesign},
    navigation::use_app_nav,
};

#[component]
pub fn Post(#[prop(optional)] id: Uuid) -> impl IntoView {
    let (title, set_title) = signal("".to_string());
    let (content, set_content) = signal("".to_string());

    let nav = use_app_nav();

    let is_create = id.is_nil();

    view! {
        <div class="flex flex-col gap-4">
            <h1 class="text-3xl">{if is_create { "Create post" } else { "Update post" }}</h1>

            <div class="flex gap-2">
                <Show when=move || {
                    is_create
                }>
                    {move || {
                        view! {
                            <Button
                                form="form-post".to_string()
                                design=ButtonDesign::Blue
                                on_click=move |_| {
                                    nav.to("/posts", NavigateOptions::default());
                                }
                            >
                                Create
                            </Button>
                        }
                    }}
                </Show>
            </div>

            <form id="form-post" class="flex flex-col gap-4 max-w-[600px]" on:submit=|e| {e.prevent_default();}>
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
