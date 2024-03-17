use leptos::{component, IntoView, view};

stylance::import_style!(#[allow(dead_code)] styles, "layout.module.css");

#[component]
pub fn Toolbox() -> impl IntoView {
    view! { <div class=styles::toolbox /> }
}