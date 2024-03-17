use leptos::{leptos_dom::logging::console_log, *};

stylance::import_style!(styles, "func_main.module.css");

#[component]
pub fn Main(children: Children) -> impl IntoView {
    view! {
        <div class=styles::func_main />
    }
}