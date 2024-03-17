use leptos::{mount_to_body, view};
use scratch::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
