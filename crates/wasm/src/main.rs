mod api;
mod app;
mod components;
mod navigation;
mod state;

use leptos::prelude::*;

use crate::app::App;

fn main() {
    mount_to_body(App);
}
