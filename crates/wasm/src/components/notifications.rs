use std::{cell::OnceCell, time::Duration};

use gloo_timers::future::sleep;
use leptos::{control_flow::Show, prelude::*, task::spawn_local};
use uuid::Uuid;

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
                        .iter()
                        .map(|n| {
                            let n = n.clone();

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

#[derive(Clone, PartialEq)]
pub enum NotificationDesign {
    Error,
    Success,
}

#[derive(Clone)]
pub struct Notification {
    pub id: Uuid,
    pub design: NotificationDesign,
    pub message: String,
}

impl Notification {
    pub fn new(design: NotificationDesign, message: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            design,
            message: message.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct NotificationsState {
    pub notifications: Vec<Notification>,
}

impl NotificationsState {
    pub fn new() -> Self {
        Self {
            notifications: vec![],
        }
    }

    pub fn add(&mut self, design: NotificationDesign, message: &str) {
        let notification = Notification::new(design, message);
        let id = notification.id.clone();

        self.notifications.push(notification);

        spawn_local(async move {
            sleep(Duration::from_secs(5)).await;
            let (_, set_notifications) = get_signal_notifications();
            set_notifications.update(|state| {
                state.notifications.retain(|n| n.id != id);
            });
        });
    }
}

type NotificationsSignal = (
    ReadSignal<NotificationsState>,
    WriteSignal<NotificationsState>,
);

thread_local! {
    static NOTIFICATIONS: OnceCell<NotificationsSignal> = OnceCell::new();
}

pub fn get_signal_notifications() -> NotificationsSignal {
    NOTIFICATIONS.with(|cell| *cell.get_or_init(|| signal(NotificationsState::new()).clone()))
}
