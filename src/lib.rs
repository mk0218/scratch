mod blocks;
mod layout;

use leptos::{component, IntoView, view};
use crate::blocks::Main;
use crate::layout::Layout;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Layout />
    }
}