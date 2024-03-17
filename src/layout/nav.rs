use leptos::{component, view, IntoView};

stylance::import_style!(#[allow(dead_code)] styles, "layout.module.css");

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class=styles::nav>
            <div>
                <h1 class=styles::title>Scratch</h1>
            </div>
            <div >
            </div>
        </div>
    }
}