use leptos::{control_flow::Show, prelude::*};

use crate::state::notifications::{NotificationDesign, get_signal_notifications};

#[component]
pub fn Notifications() -> impl IntoView {
    let (notifications, _) = get_signal_notifications();

    view! {
        <Show when=move || { !notifications.get().notifications.is_empty() }>
            <div class="fixed top-8 left-0 w-full flex justify-center">
                {move || {
                    notifications
                        .get()
                        .notifications
                        .into_iter()
                        .map(|n| {
                            view! {
                                <div class=move || {
                                    format!(
                                        "{} px-8 py-4 rounded-xl text-white max-w-md",
                                        if n.design == NotificationDesign::Error {
                                            "bg-red-800"
                                        } else {
                                            "bg-green-800"
                                        },
                                    )
                                }>{n.message}</div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </Show>
    }
}
