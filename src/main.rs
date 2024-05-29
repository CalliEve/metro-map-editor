#![allow(dead_code)] // for now while it is still in early development

use leptos::*;

mod algorithm;
mod components;
mod state;

use components::Page;
use state::StateProvider;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen max-w-screen">
            <StateProvider>
                <Page/>
            </StateProvider>
        </div>
    }
}
