use leptos::{component, view, IntoView};

stylance::import_style!(#[allow(dead_code)] styles, "layout.module.css");

#[component]
pub fn Editor() -> impl IntoView {
    view! {
        <div class=styles::editor />
    }
}