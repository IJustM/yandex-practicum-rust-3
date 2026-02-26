use gloo_net::http::{Request, RequestBuilder};
use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use leptos::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    domain::ServerError,
    state::notifications::{NotificationDesign, get_signal_notifications},
};

const API_URL: &str = std::env!("API_URL");

pub fn get_url(url: &str) -> String {
    format!("{}{}", API_URL, url)
}

const STORAGE_KEY: &str = "blog_token";

pub fn add_json_data<Data: Serialize>(req: RequestBuilder, data: Data) -> Request {
    req.header("Content-Type", "application/json")
        .json(&data)
        .unwrap()
}

pub fn add_authorization(req: RequestBuilder) -> RequestBuilder {
    req.header(
        "Authorization",
        &format!("Bearer {}", get_jwt_token().unwrap_or("".to_string())),
    )
}

pub async fn send_request<Data: DeserializeOwned>(req: Request) -> anyhow::Result<Data, ()> {
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

pub fn clear_jwt_token() {
    LocalStorage::delete(STORAGE_KEY);
}

pub fn get_jwt_token() -> Result<String, StorageError> {
    LocalStorage::get::<String>(STORAGE_KEY)
}

pub fn is_jwt_token() -> bool {
    get_jwt_token().is_ok()
}
