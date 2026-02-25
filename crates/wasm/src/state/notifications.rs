use std::{cell::OnceCell, time::Duration};

use gloo_timers::future::sleep;
use leptos::{prelude::*, task::spawn_local};
use uuid::Uuid;

use crate::state::Signal;

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
        let id = notification.id;

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

type NotificationsSignal = Signal<NotificationsState>;

thread_local! {
    static NOTIFICATIONS: OnceCell<NotificationsSignal> = const { OnceCell::new() };
}

pub fn get_signal_notifications() -> NotificationsSignal {
    NOTIFICATIONS.with(|cell| *cell.get_or_init(|| signal(NotificationsState::new())))
}
