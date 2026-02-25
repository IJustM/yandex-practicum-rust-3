use leptos::{ev::MouseEvent, prelude::*};

pub enum ButtonDesign {
    Gray,
    Blue,
    Green,
}

#[component]
pub fn Button(
    design: ButtonDesign,
    #[prop(into)] on_click: Callback<MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                format!(
                    "cursor-pointer flex-1 rounded-sm p-1 mt-4 border border-solid {}",
                    match design {
                        ButtonDesign::Gray => "bg-white text-gray-500 border-gray-500",
                        ButtonDesign::Blue => "bg-blue-500 text-white",
                        ButtonDesign::Green => "bg-green-500 text-white",
                    },
                )
            }
            on:click=move |e| { on_click.run(e) }
        >
            {children()}
        </button>
    }
}
