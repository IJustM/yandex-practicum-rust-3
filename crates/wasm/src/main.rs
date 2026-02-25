mod api;
mod app;
mod components;
mod state;
mod utils;

use leptos::prelude::*;

use crate::app::App;

fn main() {
    mount_to_body(App);
}
