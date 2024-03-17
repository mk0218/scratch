mod console;
mod editor;
mod nav;
mod toolbox;

use leptos::{component, view, IntoView};
pub use console::*;
pub use editor::*;
pub use nav::*;
pub use toolbox::*;

stylance::import_style!(#[allow(dead_code)] styles, "layout.module.css");

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class=styles::layout>
            <Nav />
            <div class=styles::container>
                <Toolbox />
                <Editor />
                <Console />
            </div>
        </div>
    }
}