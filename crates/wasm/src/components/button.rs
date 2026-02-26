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
    #[prop(optional)] is_flex_one: bool,
    #[prop(optional)] form: String,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                format!(
                    "cursor-pointer rounded-sm py-1 px-3 border border-solid {} {}",
                    match design {
                        ButtonDesign::Gray => "bg-white text-gray-500 border-gray-500",
                        ButtonDesign::Blue => "bg-blue-500 text-white",
                        ButtonDesign::Green => "bg-green-500 text-white",
                    },
                    if is_flex_one { "flex-1" } else { "" },
                )
            }
            form=if form.is_empty() { None } else { Some(form) }
            on:click=move |e| { on_click.run(e) }
        >
            {children()}
        </button>
    }
}
