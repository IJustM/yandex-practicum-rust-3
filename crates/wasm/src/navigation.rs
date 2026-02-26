use leptos::prelude::*;
use leptos_router::{NavigateOptions, hooks::use_navigate};

type NavEvent = Option<(String, NavigateOptions)>;

#[derive(Clone, Copy)]
pub struct AppNav(pub WriteSignal<NavEvent>);

impl AppNav {
    // Удобный метод для вызова из любого места
    pub fn to(&self, path: impl Into<String>, options: NavigateOptions) {
        self.0.set(Some((path.into(), options)));
    }
}

pub fn use_app_nav() -> AppNav {
    use_context::<AppNav>().expect("use_app_nav error")
}

pub fn use_init_app_nav() {
    let (path, set_path) = signal::<NavEvent>(None);

    Effect::new(move |_| {
        if let Some((new_path, navigate_options)) = path.get() {
            let navigate = use_navigate();
            navigate(&new_path, navigate_options);
            set_path.set(None); // Сбрасываем, чтобы можно было перейти по тому же пути снова
        }
    });

    provide_context(AppNav(set_path));
}
