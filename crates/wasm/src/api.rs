use gloo_net::http::{Request, RequestBuilder};
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::components::notifications::{NotificationDesign, get_signal_notifications};

const STORAGE_KEY: &str = "blog_token";

#[derive(Deserialize)]
pub struct EmptyResponse {}

pub fn add_json_data<Data: Serialize>(req: RequestBuilder, data: Data) -> Request {
    req.header("Content-Type", "application/json")
        .json(&data)
        .unwrap()
}

pub async fn send<Data: DeserializeOwned>(req: Request) -> anyhow::Result<Data, ()> {
    let res = req.send().await.unwrap();

    if res.ok() {
        let auth_response = res.json::<Data>().await.unwrap();
        Ok(auth_response)
    } else {
        let text = res.text().await.unwrap_or_default();

        let message = match serde_json::from_str::<ServerError>(&text) {
            Ok(json) => json.message,
            Err(_) => text,
        };

        let (_, set_notifications) = get_signal_notifications();
        set_notifications.update(|state| {
            state.add(NotificationDesign::Error, &message);
        });
        Err(())
    }
}

pub fn save_jwt_token(token: &str) {
    let _ = LocalStorage::set(STORAGE_KEY, token);
}

#[derive(Deserialize)]
struct ServerError {
    pub message: String,
}
