use leptos::prelude::{ReadSignal, WriteSignal};

pub mod notifications;

pub type Signal<T> = (ReadSignal<T>, WriteSignal<T>);
